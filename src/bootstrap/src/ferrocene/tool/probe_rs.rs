use std::path::PathBuf;

use crate::builder::{Builder, RunConfig, ShouldRun, Step};
use crate::core::build_steps::tool::{prepare_tool_cargo, SourceType};
use crate::core::config::TargetSelection;
use crate::{exe, Mode};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub(crate) struct ProbeRs {
    pub(crate) target: TargetSelection,
}

const PATH: &str = "ferrocene/tools/probe-rs";

impl Step for ProbeRs {
    type Output = PathBuf;
    const DEFAULT: bool = true;
    const ONLY_HOSTS: bool = true;

    fn should_run(run: ShouldRun<'_>) -> ShouldRun<'_> {
        run.path(PATH)
    }

    fn make_run(run: RunConfig<'_>) {
        run.builder.ensure(ProbeRs { target: run.target });
    }

    fn run(self, builder: &Builder<'_>) -> Self::Output {
        builder.info(format!("Building {PATH}").as_str());

        let compiler = builder.compiler(0, builder.config.build);
        let mut cmd = prepare_tool_cargo(
            builder,
            compiler,
            Mode::ToolBootstrap,
            self.target,
            "build",
            PATH,
            SourceType::Submodule,
            &[],
        )
        .into();

        builder.run(&mut cmd);

        builder
            .cargo_out(compiler, Mode::ToolBootstrap, self.target)
            .join(exe("probe-rs", self.target))
    }
}
