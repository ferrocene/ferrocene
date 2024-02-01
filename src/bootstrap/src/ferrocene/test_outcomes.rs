// SPDX-License-Identifier: MIT OR Apache-2.0
// SPDX-FileCopyrightText: The Ferrocene Developers

use crate::core::builder::{Builder, ShouldRun, Step};
use std::path::PathBuf;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub(super) struct TestOutcomesDir;

impl Step for TestOutcomesDir {
    type Output = Option<PathBuf>;

    fn should_run(run: ShouldRun<'_>) -> ShouldRun<'_> {
        run.never()
    }

    fn run(self, builder: &Builder<'_>) -> Self::Output {
        if let Some(path) = &builder.config.ferrocene_test_outcomes_dir {
            Some(std::fs::canonicalize(path).unwrap())
        } else {
            None
        }
    }
}
