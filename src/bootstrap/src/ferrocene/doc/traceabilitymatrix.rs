// SPDX-License-Identifier: MIT OR Apache-2.0
// SPDX-FileCopyrightText: The Ferrocene Developers

use crate::builder::{Builder, RunConfig, ShouldRun, Step};
use crate::core::config::TargetSelection;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub(crate) struct TraceabilityMatrix {
    target: TargetSelection,
}

impl Step for TraceabilityMatrix {
    type Output = ();
    const DEFAULT: bool = true;

    fn should_run(run: ShouldRun<'_>) -> ShouldRun<'_> {
        let builder = run.builder;
        run.path("ferrocene/tools/traceability-matrix").default_condition(builder.config.docs)
    }

    fn make_run(run: RunConfig<'_>) {
        run.builder.ensure(TraceabilityMatrix { target: run.target });
    }

    fn run(self, builder: &Builder<'_>) -> Self::Output {
        builder.ensure(crate::ferrocene::run::TraceabilityMatrix { target: self.target });
    }
}
