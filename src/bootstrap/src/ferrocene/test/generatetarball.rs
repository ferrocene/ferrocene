// SPDX-License-Identifier: MIT OR Apache-2.0
// SPDX-FileCopyrightText: The Ferrocene Developers

use crate::builder::{Builder, RunConfig, ShouldRun, Step};
use crate::core::build_steps::tool::{self, SourceType};
use crate::core::config::TargetSelection;
use crate::Mode;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub(crate) struct GenerateTarball {
    target: TargetSelection,
}

impl Step for GenerateTarball {
    type Output = ();
    const ONLY_HOSTS: bool = true;
    const DEFAULT: bool = true;

    fn run(self, builder: &Builder<'_>) {
        builder.info("test generate-tarball");

        let compiler = builder.compiler(0, self.target);
        let cargo = tool::prepare_tool_cargo(
            builder,
            compiler,
            Mode::ToolBootstrap,
            self.target,
            "test",
            "ferrocene/tools/generate-tarball",
            SourceType::InTree,
            &[],
        );
        crate::core::build_steps::test::run_cargo_test(
            cargo,
            &[],
            &[],
            "generate-tarball",
            "generate-tarball",
            compiler,
            self.target,
            builder,
        );
    }

    fn should_run(run: ShouldRun<'_>) -> ShouldRun<'_> {
        run.path("ferrocene/tools/generate-tarball")
    }

    fn make_run(run: RunConfig<'_>) {
        run.builder.ensure(Self { target: run.target });
    }
}
