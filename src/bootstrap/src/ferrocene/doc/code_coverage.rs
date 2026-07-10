// SPDX-License-Identifier: MIT OR Apache-2.0
// SPDX-FileCopyrightText: The Ferrocene Developers

use crate::FileType;
use crate::core::builder::{Builder, RunConfig, ShouldRun, Step};
use crate::core::config::{FerroceneCoverageOutcomes, TargetSelection};
use crate::ferrocene::code_coverage::CoverageOutcomesDir;

// List of targets that we generate coverage for in CI
const COVERAGE_TARGET_TRIPLES: [&'static str; 3] = [
    "aarch64-unknown-ferrocene.facade",
    "aarch64-unknown-linux-gnu",
    "x86_64-unknown-linux-gnu",
];

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub(crate) struct AllCoverageReports {
    pub(crate) target: TargetSelection,
}

impl Step for AllCoverageReports {
    type Output = ();

    fn should_run(run: ShouldRun<'_>) -> ShouldRun<'_> {
        run.alias("ferrocene-coverage")
    }

    fn is_default_step(builder: &Builder<'_>) -> bool {
        !matches!(builder.config.ferrocene_coverage_outcomes, FerroceneCoverageOutcomes::Disabled)
    }

    fn make_run(run: RunConfig<'_>) {
        run.builder.ensure(AllCoverageReports { target: run.target });
    }

    fn run(self, builder: &Builder<'_>) -> Self::Output {
        if builder.config.dry_run() {
            return;
        }
        let Some(outcomes_dir) = builder.ensure(CoverageOutcomesDir) else {
            panic!("can't generate coverage report with ferrocene.coverage-outcomes=\"disabled\"");
        };

        builder.info("Copying coverage reports...");

        for target_triple in COVERAGE_TARGET_TRIPLES {
            let out_triple = target_triple.replace("ferrocene.facade", "none");

            let src = outcomes_dir.join(target_triple).join("certified-coverage-report.html");
            let out =
                builder.doc_out(self.target).join("coverage").join(out_triple).join("index.html");

            if !src.exists() {
                panic!(
                    "`x doc ferrocene-coverage` failed: no coverage report present in {} for target {}",
                    outcomes_dir.display(),
                    target_triple
                );
            }

            builder.info(&format!("Copying {} to {}", src.display(), out.display()));
            builder.create_dir(out.parent().unwrap());
            builder.copy_link(&src, &out, FileType::Regular);
        }
    }
}
