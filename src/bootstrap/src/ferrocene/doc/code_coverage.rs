// SPDX-License-Identifier: MIT OR Apache-2.0
// SPDX-FileCopyrightText: The Ferrocene Developers

use crate::FileType;
use crate::core::builder::{Builder, RunConfig, ShouldRun, Step};
use crate::core::config::{FerroceneCoverageOutcomes, TargetSelection};
use crate::ferrocene::code_coverage::CoverageOutcomesDir;

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

        let mut saw_coverage = false;
        for entry in builder.read_dir(&outcomes_dir) {
            let name_buf = entry.file_name();
            let name = name_buf.to_str().expect("only UTF-8 file paths supported");
            assert!(name.ends_with(".html"), "unrecognized coverage report format");

            let out = builder.doc_out(self.target).join("coverage").join("index.html");
            if out.exists() {
                builder.remove(&out);
            }

            builder.create_dir(out.parent().unwrap());
            builder.info(&format!("Copying {} to {}", entry.path().display(), out.display()));
            builder.copy_link(&entry.path(), &out, FileType::Regular);
            builder.info(&format!("Generated coverage at {}", out.display()));
            saw_coverage = true;
        }

        if !saw_coverage {
            panic!(
                "`x doc ferrocene-coverage` failed: no coverage report present in {}",
                outcomes_dir.display()
            );
        }
    }
}
