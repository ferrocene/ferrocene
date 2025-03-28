use std::path::{Path, PathBuf};

use build_helper::exit;

use crate::builder::Builder;
use crate::core::build_steps::llvm::Llvm;
use crate::core::build_steps::tool::Tool;
use crate::core::builder::Cargo;
use crate::core::config::TargetSelection;
use crate::core::config::flags::FerroceneCoverageFor;
use crate::utils::build_stamp::libstd_stamp;
use crate::{BootstrapCommand, Compiler, DependencyType};

pub(crate) fn instrument_coverage(builder: &Builder<'_>, cargo: &mut Cargo) {
    if !builder.config.profiler {
        eprintln!();
        eprintln!("Error: the profiler needs to be enabled to gather coverage.");
        eprintln!("Please set `build.profiler` to `true` in your bootstrap configuration.");
        exit!(1);
    }

    cargo.rustflag("-Cinstrument-coverage");
}

pub(crate) struct GatherCoverage<'a> {
    builder: &'a Builder<'a>,
    compiler: Compiler,
    target: TargetSelection,
    coverage_for: FerroceneCoverageFor,

    profraw_dir: PathBuf,
    profdata_file: PathBuf,
    report_dir: PathBuf,
}

impl<'a> GatherCoverage<'a> {
    pub(crate) fn new(
        builder: &'a Builder<'a>,
        cargo: &mut Cargo,
        compiler: Compiler,
        target: TargetSelection,
        coverage_for: FerroceneCoverageFor,
    ) -> Self {
        let name = coverage_for.as_str();

        builder.ensure(Llvm { target });

        let profraw_dir = builder.tempdir().join(format!("ferrocene-profraw-{name}"));
        let profdata_file = builder
            .out
            .join(target.triple)
            .join("ferrocene")
            .join("coverage")
            .join(format!("{name}.profdata"));
        let report_dir = builder.doc_out(target).join("coverage").join(name);

        if profraw_dir.exists() {
            builder.remove_dir(&profraw_dir);
        }
        if profdata_file.exists() {
            builder.remove(&profdata_file);
        }
        if report_dir.exists() {
            builder.remove_dir(&report_dir);
        }

        builder.create_dir(&profraw_dir);
        builder.create_dir(profdata_file.parent().unwrap());
        builder.create_dir(&report_dir);

        let profraw_file_template = profraw_dir.join("%m_%p.profraw");
        cargo.env("LLVM_PROFILE_FILE", profraw_file_template);

        Self { builder, compiler, target, coverage_for, profraw_dir, profdata_file, report_dir }
    }

    pub(crate) fn post_process(self) {
        let coverage_dump = self.builder.tool_exe(Tool::CoverageDump);
        let llvm_bin_dir = self.builder.llvm_out(self.target).join("bin");

        if self.builder.config.dry_run() {
            return;
        }

        let mut cmd = BootstrapCommand::new(llvm_bin_dir.join("llvm-profdata"));
        cmd.arg("merge").arg("--sparse").arg("-o").arg(&self.profdata_file).arg(&self.profraw_dir);
        cmd.fail_fast().run(self.builder);

        // llvm-cov needs to receive the path to the binary that was instrumented. The path depends
        // on what we are gathering the coverage for: when adding a variant of FerroceneCoverageFor,
        // you'll need to calculate the path to the binary you called instrument_coverage() on.
        let instrumented_binary = match self.coverage_for {
            // When gathering the code coverage for the standard library, the instrumented binary is
            // the libstd-HASH.so shared library the tests link to.
            FerroceneCoverageFor::Library => {
                let mut libstd = None;
                let stamp = libstd_stamp(self.builder, self.compiler, self.target);
                for (path, kind) in self.builder.read_stamp_file(&stamp) {
                    match kind {
                        DependencyType::Host => continue,
                        DependencyType::Target | DependencyType::TargetSelfContained => {}
                    }
                    let name = path.file_name().unwrap().to_str().unwrap();
                    if name.starts_with("libstd-") && name.ends_with(".so") {
                        libstd = Some(path);
                        break;
                    }
                }
                libstd.expect("could not find libstd-HASH.so in the sysroot")
            }
        };

        let ignored_path_regexes: &[&str] = match self.coverage_for {
            FerroceneCoverageFor::Library => &[
                "\\.cargo/registry",
                "ferrocene/library/backtrace-rs",
                "ferrocene/library/libc",
                "library/alloc",
                "library/panic_unwind",
                "library/std",
            ],
        };

        let mut cmd = BootstrapCommand::new(llvm_bin_dir.join("llvm-cov"));
        cmd.arg("show").arg(instrumented_binary).arg("--instr-profile").arg(&self.profdata_file);
        cmd.arg("--format").arg("html").arg("-o").arg(self.report_dir);

        // Note that which paths are ignored changes how llvm-cov displays the paths in the report.
        // llvm-cov makes all paths relative to the common ancestor.
        for path in ignored_path_regexes {
            cmd.arg("--ignore-filename-regex").arg(path);
        }

        // By default llvm-cov includes the date the report was generated on, but that is a problem
        // for reproducibility. Disable showing the date.
        cmd.arg("--show-created-time=false");

        // Use the demangler mode of coverage-dump as the demangler. This is functionally equivalent
        // to using rustfilt, but it has the advantage of being part of the monorepo. If upstream
        // removes the demangler functionality from coverage-dump, feel free to replace the tool.
        //
        // Note that llvm-cov accepts the --Xdemangler flag multiple times. The first time it
        // specifies the binary name, and any following occurrences of --Xdemangler specifies an
        // argument provided to that binary.
        cmd.arg("--Xdemangler").arg(coverage_dump).arg("--Xdemangler").arg("--demangle");

        if !cmd.run(&self.builder) {
            let current_file = Path::new(file!())
                .strip_prefix(&self.builder.src)
                .unwrap_or(Path::new(file!()))
                .to_str()
                .unwrap()
                .to_string();
            eprintln!("Failed to run llvm-cov to generate a report!");
            eprintln!();
            eprintln!("If the error message mentions \"function name is empty\" please check the");
            eprintln!("comment at the bottom of {current_file}.");
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
