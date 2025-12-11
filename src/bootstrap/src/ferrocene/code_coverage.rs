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
use crate::ferrocene::download_and_extract_ci_outcomes;
use crate::ferrocene::run::{CertifiedCoreSymbols, CoverageReport};
use crate::{BootstrapCommand, Compiler, DocTests, Mode, t};

pub(crate) fn instrument_coverage(builder: &Builder<'_>, cargo: &mut Cargo, compiler: Compiler) {
    if !builder.config.profiler {
        eprintln!();
        eprintln!("Error: the profiler needs to be enabled to measure coverage.");
        eprintln!("Please set `build.profiler` to `true` in your bootstrap configuration.");
        exit!(1);
    }

    cargo.rustdocflag("-Cinstrument-coverage");
    cargo.rustflag("-Cinstrument-coverage");
    cargo.rustflag("--cfg=ferrocene_coverage");
    cargo.arg("--features=core/ferrocene_inject_profiler_builtins,std/ferrocene_certified_runtime");

    // Usually profiler_builtins is loaded from the sysroot, but that cannot happen when
    // building the sysroot itself: in those cases, the sysroot is empty. We thus need to
    // fetch profiler_builtins from somewhere else.
    //
    // Thankfully profiler_builtins is built as part of building the sysroot, so it will be
    // placed in the `deps` directory inside of Cargo's target directory. In theory this
    // would result in Cargo picking it up automatically, but in practice it doesn't.
    //
    // Turns out that Cargo passes `-L dependency=$target_dir/deps` to rustc instead of
    // just `-L $target_dir/deps`. The `dependency=` prefix causes rustc to only load
    // explicit dependencies from that directory, not implicitly injected crates.
    //
    // To fix the problem, we add our own `-L` flag to the Cargo invocation, pointing to
    // the location of profiler_builtins without the `dependency=` prefix.
    let target_dir =
        builder.cargo_out(compiler, Mode::Std, builder.config.host_target).join("deps");

    cargo.rustflag(&format!("-L{}", target_dir.to_str().unwrap()));
}

pub(crate) fn measure_coverage(
    builder: &Builder<'_>,
    cmd: &mut BootstrapCommand,
    compiler: Compiler,
    target: TargetSelection,
    coverage_for: FerroceneCoverageFor,
) {
    // Pre-requisites for the `generate_coverage_report()` function are built here, as that function is
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

// FIXME(@pvdrz): `blanket` needs to receive the path to the binaries that were instrumented.
// However there is no quick and easy way to fetch those. For now, we just go inside the
// dependencies and assume that every executable file is an instrumented binary.
//
// FIXME(@jyn514): This has already caused issues in practice. Running
// `x test --coverage --stage 2 && x test --coverage --stage 1` will error with
// "missing section: CoverageFunctions" because there will be  two libstds in the target folder.
// The workaround is to delete `build/host/stage1-std`.
//
// A possible improvement would be to capture `cargo test` stderr and fetch the path of every
// binary that cargo ran or get the build plan and fetch the paths of the binaries from there.
pub(super) fn instrumented_binaries(
    builder: &Builder<'_>,
    paths: &Paths,
    state: &CoverageState,
) -> Vec<PathBuf> {
    match state.coverage_for {
        FerroceneCoverageFor::Library => {
            let mut instrumented_binaries = vec![];
            let out_dir = builder.cargo_out(state.compiler, Mode::Std, state.target).join("deps");

            let res_doctest_bins = std::fs::read_dir(&paths.doctests_bins_dir);

            let doctests_bins = (builder.doc_tests != DocTests::No)
                .then(|| t!(res_doctest_bins, "cannot read doctests bins directory"))
                .into_iter()
                .flat_map(|read_dir| read_dir)
                .flat_map(|res| {
                    let path = t!(res, "cannot inspect doctest bin directory").path();
                    t!(std::fs::read_dir(path), "cannot read doctest bin directory").into_iter()
                });

            for res in
                t!(std::fs::read_dir(out_dir), "cannot read deps directory").chain(doctests_bins)
            {
                let path = t!(res, "cannot inspect deps file").path();

                #[cfg(target_os = "windows")]
                let is_executable = path.extension().is_some_and(|e| e == "exe" || e == "dll");
                #[cfg(target_family = "unix")]
                let is_executable = path.is_file() /* directories can have the executable flag set */
                        && (path.metadata().expect("cannot fetch metadata for deps file").permissions().mode() & 0o111 != 0);

                if is_executable {
                    instrumented_binaries.push(path);
                }
            }

            assert!(!instrumented_binaries.is_empty(), "could not find the instrumented binaries");
            instrumented_binaries
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
    cmd.arg("merge").arg("--sparse").arg("-o").arg(&paths.profdata_file).arg(&paths.profraw_dir);
    cmd.fail_fast().run(builder);

    let instrumented_binaries = instrumented_binaries(builder, &paths, &state);

    builder.info("Listing symbols for the certified libcore subset");
    let symbol_report =
        builder.ensure(CertifiedCoreSymbols::new(builder, builder.config.host_target));

    let html_report = builder.ensure(CoverageReport {
        certified_target: builder.config.host_target.subset_equivalent(),
        profdata: paths.profdata_file,
        instrumented_binaries,
        symbol_report,
    });

    let dist_report = paths
        .ferrocene_coverage_dir
        .join(html_report.file_name().expect("No coverage report filename determined."));
    builder.info(&format!("Saving coverage report to {}", dist_report.display()));
    builder.copy_link(&html_report, &dist_report, crate::FileType::Regular);

    if builder.doc_tests != DocTests::No {
        // Remove the doctest binaries so they're not distributed afterwards.
        t!(std::fs::remove_dir_all(&paths.doctests_bins_dir));
    }
}

fn coverage_dir(builder: &Builder<'_>, t: TargetSelection) -> PathBuf {
    builder.doc_out(t).join("coverage")
}

pub(super) fn coverage_file(builder: &Builder<'_>, t: TargetSelection) -> PathBuf {
    coverage_dir(builder, t).join("certified-coverage-report.html")
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct CoverageState {
    pub(super) target: TargetSelection,
    pub(super) compiler: Compiler,
    pub(super) coverage_for: FerroceneCoverageFor,
}
pub(crate) struct Paths {
    profraw_dir: PathBuf,
    pub(super) profdata_file: PathBuf,
    pub(crate) doctests_bins_dir: PathBuf,
    ferrocene_coverage_dir: PathBuf,
}

impl Paths {
    pub(crate) fn find(
        builder: &Builder<'_>,
        target: TargetSelection,
        coverage_for: FerroceneCoverageFor,
    ) -> Self {
        let name = format!("{}-{}", coverage_for.as_str(), target.triple);
        let out_dir = builder.out.join("ferrocene").join("coverage");
        Self {
            profraw_dir: builder.tempdir().join(format!("ferrocene-profraw-{name}")),
            profdata_file: builder.tempdir().join(format!("ferrocene-{name}.profdata")),
            doctests_bins_dir: out_dir.join("doctests-bins"),
            ferrocene_coverage_dir: out_dir,
        }
    }

    fn ensure_clean(&self, builder: &Builder<'_>) {
        // directories must be emptied, but still must exist after
        for dir in [&self.ferrocene_coverage_dir, &self.profraw_dir] {
            if dir.exists() {
                builder.remove_dir(dir);
            }
            builder.create_dir(dir);
        }

        // files only have to be deleted
        if self.profdata_file.exists() {
            builder.remove(&self.profdata_file);
        }
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
                let certified_target = builder.host_target.subset_equivalent();
                Some(coverage_dir(builder, certified_target))
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
