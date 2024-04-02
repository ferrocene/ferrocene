// SPDX-License-Identifier: MIT OR Apache-2.0
// SPDX-FileCopyrightText: The Ferrocene Developers

use crate::builder::{Builder, RunConfig, ShouldRun, Step};
use crate::core::config::TargetSelection;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub(crate) struct TechnicalReport {
    target: TargetSelection,
}

impl Step for TechnicalReport {
    type Output = ();
    const DEFAULT: bool = true;

    fn should_run(run: ShouldRun<'_>) -> ShouldRun<'_> {
        let builder = run.builder;
        run.alias("ferrocene-technical-report").default_condition(
            builder.config.docs && builder.config.ferrocene_technical_report_url.is_some(),
        )
    }

    fn make_run(run: RunConfig<'_>) {
        run.builder.ensure(TechnicalReport { target: run.target });
    }

    fn run(self, builder: &Builder<'_>) -> Self::Output {
        let url = builder
            .config
            .ferrocene_technical_report_url
            .as_deref()
            .expect("ferrocene.technical-report-url is not configured");
        let cache_path = builder
            .out
            .join("cache")
            .join("ferrocene")
            .join(url.rsplit_once('/').map(|(_, name)| name).unwrap_or(url));
        let output_dir = builder.doc_out(self.target).join("qualification");

        if builder.config.dry_run() {
            return;
        }

        if !cache_path.exists() {
            if let Some(parent) = cache_path.parent() {
                std::fs::create_dir_all(parent).unwrap();
            }
            builder.config.download_file(url, &cache_path, "");
        }

        let mut output_file = output_dir.join("technical-report.pdf");

        builder.create_dir(&output_dir);
        builder.copy_link(&cache_path, &output_file);

        // Include the technical report file only in the signatures subset.
        output_file.as_mut_os_string().push(".ferrocene-subset");
        builder.create(&output_file, "signatures");
    }
}
