// SPDX-License-Identifier: MIT OR Apache-2.0
// SPDX-FileCopyrightText: The Ferrocene Developers

use crate::core::builder::{Builder, ShouldRun, Step};
use crate::core::config::FerroceneTestOutcomes;
use std::path::PathBuf;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub(super) struct TestOutcomesDir;

impl Step for TestOutcomesDir {
    type Output = Option<PathBuf>;

    fn should_run(run: ShouldRun<'_>) -> ShouldRun<'_> {
        run.never()
    }

    fn run(self, builder: &Builder<'_>) -> Self::Output {
        match &builder.config.ferrocene_test_outcomes {
            FerroceneTestOutcomes::Disabled => None,
            FerroceneTestOutcomes::Custom(path) => Some(std::fs::canonicalize(path).unwrap()),
        }
    }
}
