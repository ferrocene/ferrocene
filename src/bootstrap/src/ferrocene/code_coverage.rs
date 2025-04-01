use std::path::PathBuf;

use build_helper::exit;

use crate::builder::Builder;
use crate::core::build_steps::llvm::Llvm;
use crate::core::builder::Cargo;
use crate::core::config::TargetSelection;
use crate::core::config::flags::FerroceneCoverageFor;
use crate::utils::build_stamp::libstd_stamp;
use crate::{BootstrapCommand, Compiler, DependencyType, t};

pub(crate) fn instrument_coverage(builder: &Builder<'_>, cargo: &mut Cargo) {
    if !builder.config.profiler {
        eprintln!();
        eprintln!("Error: the profiler needs to be enabled to measure coverage.");
        eprintln!("Please set `build.profiler` to `true` in your bootstrap configuration.");
        exit!(1);
    }

    cargo.rustflag("-Cinstrument-coverage");
}

pub(crate) fn measure_coverage(
    builder: &Builder<'_>,
    cmd: &mut BootstrapCommand,
    compiler: Compiler,
    target: TargetSelection,
    coverage_for: FerroceneCoverageFor,
) {
    // Pre-requisites for the `generate_report()` function are built here, as that function is
    // executed after all bootstrap steps are executed.
    builder.ensure(Llvm { target });

    let paths = Paths::find(builder, target, coverage_for);
    let profraw_file_template = paths.profraw_dir.join("%m_%p.profraw");
    cmd.env("LLVM_PROFILE_FILE", profraw_file_template);

    // We want to support merging the coverage information from multiple steps (for example,
    // multiple test suites), but that requires all those steps measuring the coverage of the same
    // thing. Because of that, we'll error if we already measured coverage of something and what we
    // are measuring now has a different state.
    let state = CoverageState { target, compiler, coverage_for };
    match &mut *builder.ferrocene_coverage.borrow_mut() {
        storage @ None => {
            // Only clear the paths the first time measure_coverage is called.
            paths.ensure_clean(builder);

            *storage = Some(state)
        }
        Some(existing) => {
            if state != *existing {
                eprintln!("error: cannot measure coverage in steps with different configuration!");
                eprintln!("step 1 configuration: {state:?}");
                eprintln!("step 2 configuration: {existing:?}");
                exit!(1);
            }
        }
    }
}

pub(crate) fn generate_coverage_report(builder: &Builder<'_>) {
    // Note: this function is called after all bootstrap steps are executed, to ensure the report
    // includes data from all tests suites measuring coverage. It cannot call `builder.ensure`, so
    // make sure to call it in `measure_coverage()`.

    if builder.config.cmd.ferrocene_coverage_for().is_none() {
        return;
    }
    let Some(state) = builder.ferrocene_coverage.borrow_mut().take() else {
        eprintln!("error: --coverage was passed but no steps measured coverage data.");
        exit!(1);
    };

    let paths = Paths::find(builder, state.target, state.coverage_for);
    let llvm_bin_dir = builder.llvm_out(state.target).join("bin");

    builder.info("Merging together code coverage measurements");
    let mut cmd = BootstrapCommand::new(llvm_bin_dir.join("llvm-profdata"));
    cmd.arg("merge").arg("--sparse").arg("-o").arg(&paths.profdata_file).arg(paths.profraw_dir);
    cmd.fail_fast().run(builder);

    // llvm-cov needs to receive the path to the binary that was instrumented. The path depends
    // on what we are gathering the coverage for: when adding a variant of FerroceneCoverageFor,
    // you'll need to calculate the path to the binary you called instrument_coverage() on.
    let instrumented_binary = match state.coverage_for {
        // When gathering the code coverage for the standard library, the instrumented binary is
        // the libstd-HASH.so shared library the tests link to.
        FerroceneCoverageFor::Library => {
            let mut libstd = None;
            let stamp = libstd_stamp(builder, state.compiler, state.target);
            for (path, kind) in builder.read_stamp_file(&stamp) {
                match kind {
                    DependencyType::Host => continue,
                    DependencyType::Target | DependencyType::TargetSelfContained => {}
                }
                let name = path.file_name().unwrap().to_str().unwrap();
                if name.starts_with("libstd-")
                    && (name.ends_with(".so") || name.ends_with(".dll") || name.ends_with(".dylib"))
                {
                    libstd = Some(path);
                    break;
                }
            }
            libstd.expect("could not find the libstd dynamic library in the sysroot")
        }
    };

    let ignored_path_regexes: &[&str] = match state.coverage_for {
        FerroceneCoverageFor::Library => &[
            "\\.cargo/registry",
            "ferrocene/library/backtrace-rs",
            "ferrocene/library/libc",
            "library/alloc",
            "library/panic_unwind",
            "library/std",
        ],
    };

    builder.info("Generating lcov dump of the code coverage measurements");
    let mut cmd = BootstrapCommand::new(llvm_bin_dir.join("llvm-cov"));
    cmd.arg("export").arg(instrumented_binary).arg("--instr-profile").arg(&paths.profdata_file);
    cmd.arg("--format").arg("lcov");

    // Note that which paths are ignored changes how llvm-cov displays the paths in the report.
    // llvm-cov makes all paths relative to the common ancestor.
    for path in ignored_path_regexes {
        cmd.arg("--ignore-filename-regex").arg(path);
    }

    let result = cmd.run_capture_stdout(builder);
    if result.is_failure() {
        eprintln!("Failed to run llvm-cov to generate a report!");
        eprintln!();
        eprintln!("If the error message mentions \"function name is empty\" please check the");
        eprintln!("comment at the bottom of {}.", file!());
        exit!(1);
    }

    t!(std::fs::write(&paths.lcov_file, result.stdout_bytes()));
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct CoverageState {
    target: TargetSelection,
    compiler: Compiler,
    coverage_for: FerroceneCoverageFor,
}

struct Paths {
    profraw_dir: PathBuf,
    profdata_file: PathBuf,
    lcov_file: PathBuf,
}

impl Paths {
    fn find(
        builder: &Builder<'_>,
        target: TargetSelection,
        coverage_for: FerroceneCoverageFor,
    ) -> Self {
        let name = coverage_for.as_str();
        Self {
            profraw_dir: builder.tempdir().join(format!("ferrocene-profraw-{name}")),
            profdata_file: builder.tempdir().join(format!("ferrocene-{name}.profdata")),
            lcov_file: builder
                .out
                .join(target.triple)
                .join("ferrocene")
                .join("coverage")
                .join(format!("lcov-{name}.info")),
        }
    }

    fn ensure_clean(&self, builder: &Builder<'_>) {
        if self.profraw_dir.exists() {
            builder.remove_dir(&self.profraw_dir);
        }
        if self.profdata_file.exists() {
            builder.remove(&self.profdata_file);
        }
        if self.lcov_file.exists() {
            builder.remove(&self.lcov_file);
        }

        builder.create_dir(&self.profraw_dir);
        builder.create_dir(self.lcov_file.parent().unwrap());
    }
}

/////////////////////////////////////////////////////////
//                                                     //
//   How to solve the "function name is empty" error   //
//                                                     //
/////////////////////////////////////////////////////////

// In March 2025, while developing support for code coverage for libcore, we encountered an llvm-cov
// failure with the error message "malformed instrumentation profile data: function name is empty".
//
// This failure seems to be caused by the LLVM instrumentation emitting a malformed instrumentation
// record the produced object file, tripping a validation check in llvm-cov. We are still not sure
// what caused the error, so we never fixed it.
//
// Instead, we noticed that disabling coverage instrumentation for the affected function (by adding
// the `#[coverage(off)]` attribute to it) mitigates the error. The problem then is figuring out
// which function causes the failure.
//
// The way Pietro did it in March 2025 was to patch llvm-cov to suppress the error, set the function
// name of the erroring frame to a well-known function, and seeing which function had that name in
// the report. Step-by-step instructions:
//
// 1. Get a fresh clone of LLVM:
//
//    ```
//    git clone https://github.com/llvm/llvm-project --depth 1
//    cd llvm-project
//    ```
//
// 2. Figure out in which file and line the error is emitted:
//
//    ```
//    rg "\"function name is empty\"" llvm -g "*.cpp"
//    ```
//
// 3. Replace the line returning the error with:
//
//    ```
//    FuncName = "THIS_IS_HORRIBLE";
//    ```
//
// 4. Build just llvm-cov:
//
//    ```
//    mkdir build
//    cd build
//    cmake ../llvm -DCMAKE_BUILD_TYPE=RelWithDebInfo -G Ninja
//    cmake --build . --target llvm-cov
//    ```
//
// 5. Run llvm-cov to get the source code of the broken function:
//
//    ```
//    bin/llvm-cov show --name-regex=THIS_IS_HORRIBLE --instr-profile=/path/profdata /path/object
//    ```
//
// 6. Annotate the broken function with `#[coverage(off)]`
//
// Note that the steps might have changed since the time this was written. I hope they are still an
// useful starting point for your debugging.
