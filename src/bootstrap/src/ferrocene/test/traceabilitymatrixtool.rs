// SPDX-License-Identifier: MIT OR Apache-2.0
// SPDX-FileCopyrightText: The Ferrocene Developers

use crate::builder::{Builder, RunConfig, ShouldRun, Step};
use crate::core::build_steps::tool::{self, SourceType};
use crate::core::config::TargetSelection;
use crate::Mode;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub(crate) struct TraceabilityMatrixTool {
    host: TargetSelection,
}

impl Step for TraceabilityMatrixTool {
    type Output = ();
    const DEFAULT: bool = true;
    const ONLY_HOSTS: bool = true;

    fn should_run(run: ShouldRun<'_>) -> ShouldRun<'_> {
        run.path("ferrocene/tools/traceability-matrix")
    }

    fn make_run(run: RunConfig<'_>) {
        run.builder.ensure(TraceabilityMatrixTool { host: run.target });
    }

    fn run(self, builder: &Builder<'_>) -> Self::Output {
        builder.info("Testing ferrocene/tools/traceability-matrix");
        builder.run(
            &mut tool::prepare_tool_cargo(
                builder,
                builder.compiler(0, self.host),
                Mode::ToolBootstrap,
                self.host,
                "test",
                "ferrocene/tools/traceability-matrix",
                SourceType::InTree,
                &[],
            )
            .into(),
        );
    }
}
