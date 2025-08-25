// SPDX-License-Identifier: MIT OR Apache-2.0
// SPDX-FileCopyrightText: The Ferrocene Developers

use crate::builder::{Builder, RunConfig, ShouldRun, Step};
use crate::core::build_steps::tool::{SourceType, prepare_tool_cargo};
use crate::{Kind, Mode};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub(crate) struct CoverageForSubset;

const PATH: &str = "ferrocene/tools/coverage-of-subset";

impl Step for CoverageForSubset {
    type Output = ();
    const IS_HOST: bool = true;
    const DEFAULT: bool = false;

    fn should_run(run: ShouldRun<'_>) -> ShouldRun<'_> {
        run.path(PATH)
    }

    fn make_run(run: RunConfig<'_>) {
        run.builder.ensure(Self);
    }

    fn run(self, builder: &Builder<'_>) -> Self::Output {
        builder.info(format!("Running {PATH}").as_str());

        let host_target = builder.config.host_target;

        let compiler = builder.compiler(0, host_target);
        let cmd = prepare_tool_cargo(
            builder,
            compiler,
            Mode::ToolBootstrap,
            host_target,
            Kind::Run,
            PATH,
            SourceType::InTree,
            &[],
        );
        cmd.into_cmd().run(builder);
    }
}
