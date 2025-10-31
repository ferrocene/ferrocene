// SPDX-License-Identifier: MIT OR Apache-2.0
// SPDX-FileCopyrightText: The Ferrocene Developers

use std::path::PathBuf;

use crate::builder::{Builder, RunConfig, ShouldRun, Step};
use crate::core::build_steps::compile::Std;
use crate::core::build_steps::tool::{SourceType, prepare_tool_cargo};
use crate::core::config::TargetSelection;
use crate::{Kind, Mode, exe};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub(crate) struct FlipLink {
    pub(crate) target: TargetSelection,
}

pub(in crate::ferrocene) const PATH: &str = "ferrocene/tools/flip-link";

impl Step for FlipLink {
    type Output = PathBuf;
    const DEFAULT: bool = true;
    const IS_HOST: bool = false;

    fn should_run(run: ShouldRun<'_>) -> ShouldRun<'_> {
        run.path(PATH)
    }

    fn make_run(run: RunConfig<'_>) {
        run.builder.ensure(FlipLink { target: run.target });
    }

    fn run(self, builder: &Builder<'_>) -> Self::Output {
        let compiler = builder.compiler(builder.top_stage, builder.config.host_target);
        builder.ensure(Std::new(compiler, self.target));

        builder.require_submodule("ferrocene/tools/flip-link", None);

        builder.info(format!("Building {PATH}").as_str());
        let cmd = prepare_tool_cargo(
            builder,
            compiler,
            Mode::ToolTarget,
            self.target,
            Kind::Build,
            PATH,
            SourceType::Submodule,
            &[],
        );

        cmd.into_cmd().run(builder);

        builder
            .cargo_out(compiler, Mode::ToolTarget, self.target)
            .join(exe("flip-link", self.target))
    }
}
