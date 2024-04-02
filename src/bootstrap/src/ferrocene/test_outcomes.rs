// SPDX-License-Identifier: MIT OR Apache-2.0
// SPDX-FileCopyrightText: The Ferrocene Developers

use crate::core::builder::{Builder, ShouldRun, Step};
use crate::core::config::FerroceneTestOutcomes;
use build_helper::git::get_git_merge_base;
use std::path::PathBuf;

static DOWNLOAD_PREFIX: &str = "s3://ferrocene-ci-artifacts/ferrocene/dist";

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
                let commit = get_git_merge_base(&builder.config.git_config(), None).expect(
                    "failed to retrieve the git commit for ferrocene.test-outcomes=download-ci",
                );
                Some(download_and_extract_outcomes(builder, &commit))
            }
            FerroceneTestOutcomes::Disabled => None,
            FerroceneTestOutcomes::Custom(path) => Some(std::fs::canonicalize(path).unwrap()),
        }
    }
}

fn download_and_extract_outcomes(builder: &Builder<'_>, commit: &str) -> PathBuf {
    if builder.config.dry_run() {
        return PathBuf::new();
    }

    let base = builder.out.join("cache").join("ferrocene").join("test-outcomes");
    let extracted_dir = base.join("extracted");
    let tarballs_dir = base.join("tarballs");

    let commit_file = extracted_dir.join(".ferrocene-commit");
    let tarball_file = tarballs_dir.join(format!("{commit}.tar.xz"));

    if !tarball_file.exists() {
        builder.info(&format!("Downloading test outcomes for commit {commit}"));
        let version = builder.config.artifact_version_part(commit);
        let url = format!("{DOWNLOAD_PREFIX}/{commit}/ferrocene-test-outcomes-{version}.tar.xz");
        builder.create_dir(&tarballs_dir);
        builder.config.download_file(&url, &tarball_file, "Could not download the test outcomes.");
    }

    if !commit_file.exists() || builder.read(&commit_file) != commit {
        builder.info(&format!("Extracting test outcomes for commit {commit}"));
        if extracted_dir.exists() {
            builder.remove_dir(&extracted_dir);
        }
        builder.create_dir(&extracted_dir);
        builder.config.unpack(&tarball_file, &extracted_dir, "");
        std::fs::write(&commit_file, commit.as_bytes()).unwrap();
    }

    extracted_dir.join("share").join("ferrocene").join("test-outcomes")
}
