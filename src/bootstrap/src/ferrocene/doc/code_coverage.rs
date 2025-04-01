// SPDX-License-Identifier: MIT OR Apache-2.0
// SPDX-FileCopyrightText: The Ferrocene Developers

use std::path::PathBuf;

use build_helper::exit;
use serde_derive::{Deserialize, Serialize};

use crate::core::build_steps::tool::Tool;
use crate::core::builder::{Builder, ShouldRun, Step};
use crate::core::config::TargetSelection;

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

        builder.info(&format!("Generating coverage report for {}", self.name));
        cmd.fail_fast().run(builder);

        // grcov is *extremely* annoying because it basically doesn't have any error handling,
        // neither with panics nor result enums. Instead, it just prints warnings (or sometimes not
        // even those), always generates a report (even if empty), and always exits with 0.
        //
        // The only reliable way to detect errors I found is determining how many percentage signs
        // are present in the index.html, since an empty report always contains 2, and full reports
        // contain at least another percentage for the covered file.
        let index_html = builder.read(&out.join("index.html"));
        if index_html.chars().filter(|c| *c == '%').count() <= 2 {
            eprintln!("error: grcov seems to have produced an empty report, something happened");
            exit!(1);
        }

        eprintln!("Coverage report available at file://{}/index.html", out.display());
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
