// SPDX-License-Identifier: MIT OR Apache-2.0
// SPDX-FileCopyrightText: The Ferrocene Developers

use crate::FileType;
use crate::core::builder::{Builder, RunConfig, ShouldRun, Step};
use crate::core::config::{FerroceneCoverageOutcomes, TargetSelection};
use crate::ferrocene::code_coverage::CoverageOutcomesDir;

struct CoverageTarget {
    triple: &'static str,
    doc_name: &'static str,
}

// List of targets that we generate coverage for in CI
const COVERAGE_TARGETS: [CoverageTarget; 3] = [
    CoverageTarget { triple: "x86_64-unknown-linux-gnu", doc_name: "x86_64-linux" },
    CoverageTarget { triple: "aarch64-unknown-linux-gnu", doc_name: "aarch64-linux" },
    CoverageTarget { triple: "aarch64-unknown-ferrocene.facade", doc_name: "aarch64-none" },
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

        for target in COVERAGE_TARGETS {
            let src = outcomes_dir.join(target.triple).join("certified-coverage-report.html");
            let out = builder
                .doc_out(self.target)
                .join("coverage")
                .join(target.doc_name)
                .join("index.html");

            if !src.exists() {
                panic!(
                    "`x doc ferrocene-coverage` failed: no coverage report present in {} for target {}",
                    outcomes_dir.display(),
                    target.triple
                );
            }

            builder.create_dir(out.parent().unwrap());
            builder.info(&format!("Copying {} to {}", src.display(), out.display()));
            builder.copy_link(&src, &out, FileType::Regular);
            builder.info(&format!("Generated coverage at {}", out.display()));
        }
    }
}
