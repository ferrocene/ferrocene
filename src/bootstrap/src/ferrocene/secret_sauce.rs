use std::path::PathBuf;

use crate::TargetSelection;
use crate::core::builder::{Builder, ShouldRun, Step};
use crate::core::config::FerroceneSecretSauce;

static DOWNLOAD_PREFIX: &str = "s3://ferrocene-ci-mirrors/coretest-secret-sauce/20250130";
const COMMIT: &str = "bacd53d";

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub(crate) struct SecretSauceArtifacts {
    pub target: TargetSelection,
}

impl Step for SecretSauceArtifacts {
    type Output = PathBuf;

    fn should_run(run: ShouldRun<'_>) -> ShouldRun<'_> {
        run.never()
    }

    fn run(self, builder: &Builder<'_>) -> Self::Output {
        if builder.config.dry_run() {
            return PathBuf::new();
        }

        if let FerroceneSecretSauce::Local(secret_sauce_dir) =
            &builder.config.ferrocene_secret_sauce
        {
            secret_sauce_dir.join(&self.target)
        } else {
            download_and_extract_secret_sauce(builder, &self.target)
        }
    }
}

fn download_and_extract_secret_sauce(builder: &Builder<'_>, target: &TargetSelection) -> PathBuf {
    let base = builder.out.join("cache").join("ferrocene").join("secret-sauce");
    let extracted_dir = base.join("extracted").join(target);
    let tarballs_dir = base.join("tarballs").join(COMMIT);

    let commit_file = extracted_dir.join(".secret-sauce-commit");
    let tarball_filename = format!("{target}.tar.xz");
    let tarball_file = tarballs_dir.join(&tarball_filename);

    if !tarball_file.exists() {
        builder.info(&format!("Downloading secret-sauce for target {target}"));
        let url = format!("{DOWNLOAD_PREFIX}/{COMMIT}/{tarball_filename}");
        builder.create_dir(&tarballs_dir);
        builder.config.download_file(&url, &tarball_file, "Could not download secret-sauce.");
    }

    if !commit_file.exists() || builder.read(&commit_file) != COMMIT {
        builder.info(&format!("Extracting secret-sauce for target {target}"));
        builder.create_dir(&extracted_dir);
        builder.config.unpack(&tarball_file, &extracted_dir, "");
        std::fs::write(&commit_file, COMMIT.as_bytes()).unwrap();
    }

    extracted_dir
}
