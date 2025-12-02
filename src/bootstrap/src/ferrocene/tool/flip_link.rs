// SPDX-License-Identifier: MIT OR Apache-2.0
// SPDX-FileCopyrightText: The Ferrocene Developers

use std::path::PathBuf;

use crate::builder::{Builder, RunConfig, ShouldRun, Step};
use crate::core::build_steps::tool::{SourceType, prepare_tool_cargo};
use crate::core::config::TargetSelection;
use crate::{Kind, Mode, exe};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub(crate) struct FlipLink {
    pub(crate) target: TargetSelection,
}

const PATH: &str = "ferrocene/tools/flip-link";

impl Step for FlipLink {
    type Output = PathBuf;
    const IS_HOST: bool = true;

    fn should_run(run: ShouldRun<'_>) -> ShouldRun<'_> {
        run.path(PATH)
    }

    fn is_default_step(_: &Builder<'_>) -> bool {
        true
    }

    fn make_run(run: RunConfig<'_>) {
        run.builder.ensure(FlipLink { target: run.target });
    }

    fn run(self, builder: &Builder<'_>) -> Self::Output {
        builder.info(format!("Building {PATH}").as_str());

        let compiler = builder.compiler(0, builder.config.host_target);
        let cmd = prepare_tool_cargo(
            builder,
            compiler,
            Mode::ToolBootstrap,
            self.target,
            Kind::Build,
            PATH,
            SourceType::Submodule,
            &[],
        );

        cmd.into_cmd().run(builder);

        builder
            .cargo_out(compiler, Mode::ToolBootstrap, self.target)
            .join(exe("flip-link", self.target))
    }
}
