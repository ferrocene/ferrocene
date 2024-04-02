// SPDX-License-Identifier: MIT OR Apache-2.0
// SPDX-FileCopyrightText: The Ferrocene Developers

use crate::builder::{Builder, RunConfig, ShouldRun, Step};
use crate::core::build_steps::tool::{self, SourceType};
use crate::core::config::TargetSelection;
use crate::Mode;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub(crate) struct SelfTest {
    target: TargetSelection,
}

impl Step for SelfTest {
    type Output = ();
    const DEFAULT: bool = true;
    const ONLY_HOSTS: bool = true;

    fn should_run(run: ShouldRun<'_>) -> ShouldRun<'_> {
        run.path("ferrocene/tools/self-test")
    }

    fn make_run(run: RunConfig<'_>) {
        run.builder.ensure(SelfTest { target: run.target });
    }

    fn run(self, builder: &Builder<'_>) -> Self::Output {
        builder.info("Testing ferrocene/tools/self-test");

        let mut cmd = tool::prepare_tool_cargo(
            builder,
            builder.compiler(0, self.target),
            Mode::ToolBootstrap,
            self.target,
            "test",
            "ferrocene/tools/self-test",
            SourceType::InTree,
            &[],
        )
        .into();
        crate::ferrocene::tool::SelfTest::update_command(&mut cmd, builder, self.target);
        builder.run(&mut cmd);
    }
}
