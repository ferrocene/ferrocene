use std::path::Path;

use crate::builder::{Builder, RunConfig, ShouldRun, Step};
use crate::core::build_steps::tool::SourceType;
use crate::core::config::TargetSelection;
use crate::BootstrapCommand;
use crate::Command;
use crate::Mode;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub(crate) struct ProfilerBuiltinsNoCore {
    pub(crate) target: TargetSelection,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub(crate) struct InstrumentationCoverageFlags {
    rustflags: Vec<String>,
}

impl InstrumentationCoverageFlags {
    pub fn flags(&self) -> &[String] {
        &self.rustflags
    }
}

impl Step for ProfilerBuiltinsNoCore {
    type Output = InstrumentationCoverageFlags;
    const ONLY_HOSTS: bool = true;
    const DEFAULT: bool = true;

    fn run(self, builder: &Builder<'_>) -> Self::Output {
        builder.info("Building profiler_builtins with no_core");

        let target = self.target;
        let compiler = builder.compiler(1, target);

        // x.py test --coverage
        // [ProfilerBuiltinNoCore step] Builds profiler_builtins with no core -> profiler_builtins_no_core.rlib
        // [Core Tests step] Compile tests of core with -Cinsturment-coverage -> link against profiler_builtins_no_core.rlib

        let mut cargo = builder.cargo(compiler, Mode::Std, SourceType::InTree, target, "build");

        cargo.current_dir(Path::new("library/profiler_builtins"));

        let profiler_builtins_no_core_dir = "profiler_builtins_no_core";
        let target_dir =
            builder.cargo_out(compiler, Mode::Std, target).join(profiler_builtins_no_core_dir);

        cargo.arg("--target-dir");
        cargo.arg(&*target_dir.to_string_lossy());
        cargo.arg("--no-default-features");

        let mut cmd: Command = cargo.into();
        builder.run_cmd(BootstrapCommand::from(&mut cmd).fail_fast());

        let cargo_dir = if builder.config.rust_optimize.is_release() { "release" } else { "debug" };

        let lib_dir = target_dir.join(&*target.triple).join(cargo_dir);
        let lib_path = lib_dir.join("libprofiler_builtins.rlib");
        let profiler_builtins_no_core_path = lib_dir.join("libprofiler_builtins_no_core.rlib");

        if !builder.config.dry_run() {
            std::fs::rename(lib_path, &profiler_builtins_no_core_path)
                .expect("Could not rename the profiler_builtins_no_core library");
            assert!(
                profiler_builtins_no_core_path.exists() && profiler_builtins_no_core_path.is_file()
            );
        }
        let mut instrument_coverage_flags = InstrumentationCoverageFlags { rustflags: Vec::new() };

        instrument_coverage_flags.rustflags.push("-Cinstrument-coverage".into());
        instrument_coverage_flags.rustflags.push("--extern".into());
        instrument_coverage_flags.rustflags.push(format!(
            "profiler_builtins_no_core={}",
            profiler_builtins_no_core_path.to_str().unwrap()
        ));
        instrument_coverage_flags
            .rustflags
            .push("-Zprofiler_runtime=profiler_builtins_no_core".into());
        instrument_coverage_flags.rustflags.push("-L".into());
        instrument_coverage_flags.rustflags.push(lib_dir.to_str().unwrap().to_string());

        instrument_coverage_flags
    }

    fn should_run(run: ShouldRun<'_>) -> ShouldRun<'_> {
        run.never()
    }

    fn make_run(run: RunConfig<'_>) {
        run.builder.ensure(Self { target: run.target });
    }
}
