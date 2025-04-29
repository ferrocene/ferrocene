// SPDX-License-Identifier: MIT OR Apache-2.0
// SPDX-FileCopyrightText: The Ferrocene Developers

use std::path::PathBuf;

use crate::core::builder::{Builder, ShouldRun, Step};
use crate::core::config::FerroceneTestOutcomes;
use crate::ferrocene::download_and_extract_ci_outcomes;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub(super) struct TestOutcomesDir;

impl Step for TestOutcomesDir {
    type Output = Option<PathBuf>;

    fn should_run(run: ShouldRun<'_>) -> ShouldRun<'_> {
        run.never()
    }

    fn run(self, builder: &Builder<'_>) -> Self::Output {
        match &builder.config.ferrocene_test_outcomes {
            FerroceneTestOutcomes::DownloadCi => {
                Some(download_and_extract_ci_outcomes(builder, "test"))
            }
            FerroceneTestOutcomes::Disabled => None,
            FerroceneTestOutcomes::Custom(path) => Some(std::fs::canonicalize(path).unwrap()),
            FerroceneTestOutcomes::Local => {
                let metrics = builder.out.join("metrics.json");
                let local_path = builder.out.join("ferrocene").join("local-test-outcomes");
                if !builder.config.dry_run() {
                    if local_path.exists() {
                        builder.remove_dir(&local_path);
                    }
                    builder.create_dir(&local_path);
                    if metrics.exists() {
                        std::fs::copy(metrics, local_path.join("local.json")).unwrap();
                    }
                }
                Some(local_path)
            }
        }
    }
}
