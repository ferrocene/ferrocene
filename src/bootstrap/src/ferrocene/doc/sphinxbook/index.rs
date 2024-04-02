// SPDX-License-Identifier: MIT OR Apache-2.0
// SPDX-FileCopyrightText: The Ferrocene Developers

use crate::builder::{Builder, RunConfig, ShouldRun, Step};
use crate::core::config::TargetSelection;

use super::BreadcrumbsAssets;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub(crate) struct Index {
    pub(super) target: TargetSelection,
}

impl Step for Index {
    type Output = ();
    const DEFAULT: bool = true;

    fn should_run(run: ShouldRun<'_>) -> ShouldRun<'_> {
        let builder = run.builder;
        run.path("ferrocene/doc/index").default_condition(builder.config.docs)
    }

    fn make_run(run: RunConfig<'_>) {
        run.builder.ensure(Index { target: run.target });
    }

    fn run(self, builder: &Builder<'_>) -> Self::Output {
        builder.ensure(BreadcrumbsAssets { target: self.target, dest: None });
        builder.cp_link_r(
            &builder.src.join("ferrocene").join("doc").join("index"),
            &builder.out.join(self.target.triple).join("doc"),
        );
    }
}
