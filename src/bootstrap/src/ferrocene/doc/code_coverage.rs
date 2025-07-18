// SPDX-License-Identifier: MIT OR Apache-2.0
// SPDX-FileCopyrightText: The Ferrocene Developers

use std::error::Error;
use std::path::PathBuf;

use build_helper::exit;
use serde_derive::{Deserialize, Serialize};

use crate::core::build_steps::tool::Tool;
use crate::core::builder::{Builder, RunConfig, ShouldRun, Step};
use crate::core::config::{FerroceneCoverageOutcomes, TargetSelection};
use crate::ferrocene::code_coverage::CoverageOutcomesDir;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub(crate) struct AllCoverageReports {
    pub(crate) target: TargetSelection,
}

impl Step for AllCoverageReports {
    type Output = ();
    const DEFAULT: bool = true;

    fn should_run(run: ShouldRun<'_>) -> ShouldRun<'_> {
        let default = !matches!(
            run.builder.config.ferrocene_coverage_outcomes,
            FerroceneCoverageOutcomes::Disabled
        );
        run.alias("ferrocene-coverage").default_condition(default)
    }

    fn make_run(run: RunConfig<'_>) {
        run.builder.ensure(AllCoverageReports { target: run.target });
    }

    fn run(self, builder: &Builder<'_>) -> Self::Output {
        let Some(outcomes_dir) = builder.ensure(CoverageOutcomesDir) else {
            panic!("can't generate coverage report with ferrocene.coverage-outcomes=\"disabled\"");
        };

        for entry in builder.read_dir(&outcomes_dir) {
            let Some(name) = entry
                .file_name()
                .to_str()
                .and_then(|s| s.strip_prefix("lcov-"))
                .and_then(|s| s.strip_suffix(".info"))
                .map(|s| s.to_string())
            else {
                continue;
            };

            let metadata = entry.path().parent().unwrap().join(format!("metadata-{name}.json"));
            if !metadata.exists() {
                panic!("lcov file {:?} without corresponding metadata", entry.path().display());
            }
            // We are not using builder.read here because that function doesn't do anything during a
            // dry run. Instead, we do want the file to be read during the dry run, because it
            // affects the step we have to execute.
            let metadata: CoverageMetadata = match std::fs::read(&metadata)
                .map_err(box_err)
                .and_then(|raw| serde_json::from_slice(&raw).map_err(box_err))
            {
                Ok(parsed) => parsed,
                Err(err) => panic!("failed to parse {:?}: {err:?}", metadata.display()),
            };

            builder.ensure(SingleCoverageReport {
                target: self.target,
                name,
                lcov: entry.path(),
                metadata,
            });
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub(crate) struct SingleCoverageReport {
    pub(crate) target: TargetSelection,
    pub(crate) name: String,
    pub(crate) lcov: PathBuf,
    pub(crate) metadata: CoverageMetadata,
}

impl Step for SingleCoverageReport {
    type Output = ();

    fn should_run(run: ShouldRun<'_>) -> ShouldRun<'_> {
        run.never()
    }

    fn run(self, builder: &Builder<'_>) -> Self::Output {
        if self.metadata.metadata_version != CoverageMetadata::CURRENT_VERSION {
            panic!(
                "coverage metadata version {} (in {}) is not supported",
                self.metadata.metadata_version, self.name
            );
        }

        let out = builder.doc_out(self.target).join("coverage").join(&self.name);
        if out.exists() {
            builder.remove_dir(&out);
        }

        let mut cmd = builder.tool_cmd(Tool::FerroceneGrcov);
        cmd.arg(&self.lcov).arg("--output-types=html").arg("--output-path").arg(&out);

        // Avoid including the current date and time in the report.
        cmd.arg("--no-date");

        // We are using `current_dir()` instead of passing the `--source-dir` flag because the
        // latter does extra work to support Java projects, causing a huge performance decrease
        // (from 60ms to 2.8 seconds on Pietro's computer).
        cmd.current_dir(&builder.src);

        // llvm-cov always includes absolute paths in its reports, which don't work outside of the
        // system llvm-cov is executed on. We get the prefix from the coverage metadata, and tell
        // grcov to strip it.
        cmd.arg("--prefix-dir").arg(&self.metadata.path_prefix);

        // If collecting library coverage, ignore all directories that are not
        // part of libcore. Otherwise things like compiler-builtins, coretests
        // etc. will be included. portable-simd and stdarch are kept as well,
        // since parts of them are included into libcore via a path attribute.
        if self.name.starts_with("library") {
            cmd.args(["--keep-only", "library/{core,portable-simd,stdarch}/**"]);
        }

        builder.info(&format!("Generating coverage report for {}", self.name));
        cmd.fail_fast().run(builder);

        // grcov is *extremely* annoying because it basically doesn't have any error handling,
        // neither with panics nor result enums. Instead, it just prints warnings (or sometimes not
        // even those), always generates a report (even if empty), and always exits with 0.
        //
        // The only reliable way to detect errors I found is determining how many percentage signs
        // are present in the index.html, since an empty report always contains 2, and full reports
        // contain at least another percentage for the covered file.
        if !builder.config.dry_run() {
            let index_html = builder.read(&out.join("index.html"));
            if index_html.chars().filter(|c| *c == '%').count() <= 2 {
                eprintln!(
                    "error: grcov seems to have produced an empty report, something happened"
                );
                exit!(1);
            }
        }

        builder.info(&format!("Coverage report available at file://{}/index.html", out.display()));
    }
}

/// Some metadata needed to generate the coverage report is not present in the lcov file, so we need
/// to store it in an auxiliary file.
#[derive(Debug, Serialize, Deserialize, Clone, Hash, PartialEq, Eq)]
pub(crate) struct CoverageMetadata {
    pub(crate) metadata_version: u32,
    pub(crate) path_prefix: PathBuf,
}

impl CoverageMetadata {
    pub(crate) const CURRENT_VERSION: u32 = 1;
}

fn box_err<E: Error + 'static>(err: E) -> Box<dyn Error> {
    Box::new(err)
}
