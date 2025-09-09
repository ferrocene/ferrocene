// SPDX-License-Identifier: MIT OR Apache-2.0
// SPDX-FileCopyrightText: The Ferrocene Developers

#[cfg(target_family = "unix")]
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;

use build_helper::exit;

use crate::builder::Builder;
use crate::core::build_steps::llvm::Llvm;
use crate::core::builder::{Cargo, ShouldRun, Step};
use crate::core::config::flags::FerroceneCoverageFor;
use crate::core::config::{FerroceneCoverageOutcomes, TargetSelection};
use crate::ferrocene::doc::code_coverage::{CoverageMetadata, SingleCoverageReport};
use crate::ferrocene::download_and_extract_ci_outcomes;
use crate::{BootstrapCommand, Compiler, GitRepo, Mode, RemapScheme, t};

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

    // FIXME(@pvdrz): llvm-cov needs to receive the path to the binaries that were instrumented.
    // However there is no quick and easy way to fetch those. For now, we just go inside the
    // dependencies and assume that every executable file is an instrumented binary.
    //
    // A possible improvement would be to capture `cargo test` stderr and fetch the path of every
    // binary that cargo ran or get the build plan and fetch the paths of the binaries from there.
    let instrumented_binaries = match state.coverage_for {
        FerroceneCoverageFor::Library => {
            let mut instrumented_binaries = vec![];
            let out_dir = builder.cargo_out(state.compiler, Mode::Std, state.target).join("deps");
            for res in std::fs::read_dir(out_dir).expect("cannot read deps directory") {
                let path = res.expect("cannot inspect deps file").path();

                #[cfg(target_os = "windows")]
                let is_executable = path.extension().is_some_and(|e| e == "exe");
                #[cfg(target_family = "unix")]
                let is_executable = path.is_file() /* directories can have the executable flag set */
                    && path.extension().is_none() /* filter `.so` files */
                    && (path.metadata().expect("cannot fetch metadata for deps file").permissions().mode() & 0o111 != 0);

                if is_executable {
                    instrumented_binaries.push(path);
                }
            }
            assert!(!instrumented_binaries.is_empty(), "could not find the instrumented binaries");
            instrumented_binaries
        }
    };

    let ignored_path_regexes: &[&str] = match state.coverage_for {
        FerroceneCoverageFor::Library => &[
            // Ignore Cargo dependencies:
            "\\.cargo/registry", // Without remap-path-prefix
            "/rust/deps",        // With remap-path-prefix
            // Ignore files we don't currently handle:
            "ferrocene/library/backtrace-rs",
            "ferrocene/library/libc",
            "library/alloc",
            "library/panic_unwind",
            "library/std",
        ],
    };

    builder.info("Generating lcov dump of the code coverage measurements");
    let mut cmd = BootstrapCommand::new(llvm_bin_dir.join("llvm-cov"));
    cmd.arg("export").args(instrumented_binaries).arg("--instr-profile").arg(&paths.profdata_file);
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

    let metadata = CoverageMetadata {
        metadata_version: CoverageMetadata::CURRENT_VERSION,
        path_prefix: if let Some(path) =
            builder.debuginfo_map_to(GitRepo::Rustc, RemapScheme::NonCompiler)
        {
            path.into()
        } else {
            builder.src.clone()
        },
    };

    t!(std::fs::write(&paths.lcov_file, result.stdout_bytes()));
    t!(std::fs::write(&paths.metadata_file, &t!(serde_json::to_vec_pretty(&metadata))));

    if builder.config.ferrocene_generate_coverage_report_after_tests {
        builder.ensure(SingleCoverageReport {
            target: state.target,
            name: format!("{}-{}", state.coverage_for.as_str(), state.target.triple),
            lcov: paths.lcov_file,
            metadata,
        });
    }
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
    metadata_file: PathBuf,
}

impl Paths {
    fn find(
        builder: &Builder<'_>,
        target: TargetSelection,
        coverage_for: FerroceneCoverageFor,
    ) -> Self {
        let name = format!("{}-{}", coverage_for.as_str(), target.triple);
        let out_dir = builder.out.join("ferrocene").join("coverage");
        Self {
            profraw_dir: builder.tempdir().join(format!("ferrocene-profraw-{name}")),
            profdata_file: builder.tempdir().join(format!("ferrocene-{name}.profdata")),
            lcov_file: out_dir.join(format!("lcov-{name}.info")),
            metadata_file: out_dir.join(format!("metadata-{name}.json")),
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
        if self.metadata_file.exists() {
            builder.remove(&self.metadata_file);
        }

        builder.create_dir(&self.profraw_dir);
        builder.create_dir(self.lcov_file.parent().unwrap());
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub(crate) struct CoverageOutcomesDir;

impl Step for CoverageOutcomesDir {
    type Output = Option<PathBuf>;

    fn should_run(run: ShouldRun<'_>) -> ShouldRun<'_> {
        run.never()
    }

    fn run(self, builder: &Builder<'_>) -> Self::Output {
        match &builder.config.ferrocene_coverage_outcomes {
            FerroceneCoverageOutcomes::Disabled => None,
            FerroceneCoverageOutcomes::DownloadCi => {
                Some(download_and_extract_ci_outcomes(builder, "coverage"))
            }
            FerroceneCoverageOutcomes::Local => {
                Some(builder.out.join("ferrocene").join("coverage"))
            }
            FerroceneCoverageOutcomes::Custom(path) => Some(path.clone()),
        }
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
