// SPDX-License-Identifier: MIT OR Apache-2.0
// SPDX-FileCopyrightText: The Ferrocene Developers

use crate::builder::{Builder, ShouldRun, Step};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub(super) struct EmptyStep;

impl Step for EmptyStep {
    type Output = ();

    fn should_run(run: ShouldRun<'_>) -> ShouldRun<'_> {
        run.never()
    }

    fn run(self, _: &Builder<'_>) -> Self::Output {}
}
