use std::path::PathBuf;

use build_helper::exit;

use crate::BootstrapCommand;
use crate::builder::Builder;
use crate::core::build_steps::llvm::Llvm;
use crate::core::builder::Cargo;
use crate::core::config::TargetSelection;

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
    cargo: &mut Cargo,
    target: TargetSelection,
    name: &str,
) {
    // Pre-requisites for the `generate_report()` function are built here, as that function is
    // executed after all bootstrap steps are executed.
    builder.ensure(Llvm { target });

    let paths = Paths::find(builder, target, name);
    let profraw_file_template = paths.profraw_dir.join("%m_%p.profraw");
    cargo.env("LLVM_PROFILE_FILE", profraw_file_template);

    instrument_coverage(builder, cargo);

    // We want to support merging the coverage information from multiple steps (for example,
    // multiple test suites), but that requires all those steps measuring the coverage of the same
    // thing. Because of that, we'll error if we already measured coverage of something and what we
    // are measuring now has a different state.
    let state = CoverageState { target, name: name.into() };
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

    if !builder.config.cmd.coverage() {
        return;
    }
    let Some(state) = builder.ferrocene_coverage.borrow_mut().take() else {
        eprintln!("error: --coverage was passed but no steps measured coverage data.");
        exit!(1);
    };

    let paths = Paths::find(builder, state.target, &state.name);
    let llvm_bin_dir = builder.llvm_out(state.target).join("bin");

    let mut cmd = BootstrapCommand::new(llvm_bin_dir.join("llvm-profdata"));
    cmd.arg("merge").arg("--sparse").arg("-o").arg(paths.profdata_file).arg(paths.profraw_dir);
    cmd.fail_fast().run(builder);
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct CoverageState {
    target: TargetSelection,
    name: String,
}

struct Paths {
    profraw_dir: PathBuf,
    profdata_file: PathBuf,
}

impl Paths {
    fn find(builder: &Builder<'_>, target: TargetSelection, name: &str) -> Self {
        Self {
            profraw_dir: builder.tempdir().join(format!("ferrocene-profraw-{name}")),
            profdata_file: builder
                .out
                .join(target.triple)
                .join("ferrocene")
                .join("coverage")
                .join(format!("{name}.profdata")),
        }
    }

    fn ensure_clean(&self, builder: &Builder<'_>) {
        if self.profraw_dir.exists() {
            builder.remove_dir(&self.profraw_dir);
        }
        if self.profdata_file.exists() {
            builder.remove(&self.profdata_file);
        }

        builder.create_dir(&self.profraw_dir);
        builder.create_dir(self.profdata_file.parent().unwrap());
    }
}
