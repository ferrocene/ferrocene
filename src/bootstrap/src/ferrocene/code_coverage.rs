use crate::builder::{Builder, RunConfig, ShouldRun, Step};
use crate::Command;
use crate::BootstrapCommand;
use crate::Mode;
use crate::core::build_steps::tool::SourceType;
use std::path::{Path, PathBuf};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub(crate) struct ProfilerBuiltinsNoCore;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub(crate) struct ProfilerBuiltins;

impl Step for ProfilerBuiltinsNoCore {
    type Output = PathBuf;
    const ONLY_HOSTS: bool = true;
    const DEFAULT: bool = true;

    fn run(self, builder: &Builder<'_>) -> Self::Output {
        builder.info("Building profiler_builtins with no_core");
        
        let target = builder.config.build;
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

        if !builder.config.dry_run() {
            assert!(lib_path.exists() && lib_path.is_file());
        }
        lib_path
    }

    fn should_run(run: ShouldRun<'_>) -> ShouldRun<'_> {
        run.never()
    }

    fn make_run(run: RunConfig<'_>) {
        run.builder.ensure(Self {});
    }
}
