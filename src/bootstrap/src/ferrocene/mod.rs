// SPDX-License-Identifier: MIT OR Apache-2.0
// SPDX-FileCopyrightText: The Ferrocene Developers

pub(crate) mod code_coverage;
pub(crate) mod dist;
pub(crate) mod doc;
pub(crate) mod install;
pub(crate) mod partners;
pub(crate) mod run;
pub(crate) mod secret_sauce;
pub(crate) mod sign;
pub(crate) mod test;
pub(crate) mod test_outcomes;
pub(crate) mod test_variants;
pub(crate) mod tool;

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use build_helper::ci::CiEnv;
use build_helper::git::get_closest_upstream_commit;

use crate::builder::Builder;
use crate::core::config::{Config, TargetSelection};
use crate::t;
use crate::utils::exec::{BootstrapCommand, command};

/// Helper function used to download files from S3. This is used to be able to download artifacts
/// from our buckets for download-ci-llvm and download-rustc.
pub(crate) fn download_from_s3(config: &Config, url: &str, tempfile: &Path, help_on_error: &str) {
    // If no profile has been configured, do not pass a profile rather than passing the profile
    // called "default". If we pass "default" as the profile, it will indeed take the default
    // profile from ~/.aws/credentials, but it will ignore environment variables. CI credentials
    // are provided through environment variables.
    let tmp_binding;
    let profile_flags = match config.ferrocene_aws_profile.as_deref() {
        Some(profile) => {
            tmp_binding = ["--profile", profile];
            &tmp_binding as &[_]
        }
        None => &[],
    };

    let success =
        command("aws").args(["s3", "cp"]).args(profile_flags).arg(url).arg(tempfile).run(config);

    if !success {
        if !help_on_error.is_empty() {
            eprintln!("{help_on_error}");
        }
        eprintln!();
        eprintln!("Note that on Ferrocene, interaction with our storage are only available to");
        eprintln!("Ferrous Systems employees or contractors working on Ferrocene.");
        eprintln!();
        eprintln!("You might also need to authenticate with AWS if you haven't done so today:");
        eprintln!();
        if let Some(profile) = config.ferrocene_aws_profile.as_deref() {
            eprintln!("    aws sso login --profile {profile}");
        } else {
            eprintln!("    aws sso login");
            eprintln!();
            eprintln!("You don't seem to have an AWS profile configured. If you have access to");
            eprintln!("Ferrous Systems's AWS environment, please follow the onboarding steps.");
        }
        eprintln!();
        std::process::exit(1);
    }
}

pub(crate) fn download_from_local_filesystem(path: &str, dest: &Path, help_on_error: &str) {
    if let Err(err) = std::fs::copy(path, dest) {
        eprintln!("Failed to copy {path}: {err}");
        if !help_on_error.is_empty() {
            eprintln!("{help_on_error}");
        }
        std::process::exit(1);
    }
}

pub(crate) fn ignored_tests_for_suite(
    builder: &Builder<'_>,
    target: TargetSelection,
    suite: &str,
) -> Vec<String> {
    #[derive(serde_derive::Deserialize)]
    struct Item {
        tests: Vec<String>,
        targets: Vec<String>,
        // Other fields ignored here...
    }

    let contents: HashMap<String, Vec<Item>> = t!(toml::from_slice(&t!(std::fs::read(
        builder.src.join("ferrocene").join("ignored-tests.toml")
    ))));

    let triple = target.triple.to_string();
    contents
        .get(suite)
        .map(|s| s.as_slice())
        .unwrap_or(&[])
        .iter()
        .filter(|item| item.targets.contains(&triple))
        .flat_map(|item| item.tests.iter())
        .map(|i| i.clone())
        .collect()
}

fn ferrocene_channel(builder: &Builder<'_>, ferrocene_version: &str) -> String {
    match (&*builder.config.channel, &*builder.config.ferrocene_raw_channel) {
        ("nightly" | "dev", "rolling") => "nightly".to_owned(),
        ("beta", "rolling") => "pre-rolling".to_owned(),
        ("stable", "rolling") => "rolling".to_owned(),
        ("stable", ferrocene_channel @ ("beta" | "stable")) => {
            let major_ferrocene = (|| {
                let mut version_components = ferrocene_version.split('.');
                let year = version_components.next()?;
                let month = version_components.next()?;
                let _patch = version_components.next()?;
                if version_components.next().is_none() {
                    Some(format!("{year}.{month}"))
                } else {
                    None
                }
            })();
            match major_ferrocene {
                Some(major_ferrocene) => format!("{ferrocene_channel}-{major_ferrocene}"),
                None => panic!(
                    "invalid ferrocene/version, expected 'year.month.patch', got: {ferrocene_version}"
                ),
            }
        }
        (rust, ferrocene) => panic!(
            "error: unsupported channel configuration: rust '{rust}' and ferrocene '{ferrocene}'"
        ),
    }
}

fn uv_command(builder: &Builder<'_>) -> BootstrapCommand {
    let uv = builder.config.uv.as_ref().expect("uv is required");
    let python = builder.config.python.as_ref().expect("python is required");

    let mut command = BootstrapCommand::new(&uv);

    // Prevent uv from managing its own Python version. Instead, let's be consistent with the rest
    // of bootstrap and use the explicit `build.python` in `config.toml` (or the default one).
    command
        .env("UV_PYTHON", &python)
        .env("UV_PYTHON_PREFERENCE", "only-system")
        .env("UV_PYTHON_DOWNLOADS", "never");

    // Handle vendored dependencies.
    if builder.config.vendor {
        command
            .env("UV_OFFLINE", "1")
            // uv doesn't have a native way to vendor dependencies. To work around that, our vendor
            // command populates the uv cache, and we reuse it here.
            .env("UV_CACHE_DIR", builder.src.join("vendor").join("uv"));
    }

    command
}

fn download_and_extract_ci_outcomes(builder: &Builder<'_>, kind: &str) -> PathBuf {
    if builder.config.dry_run() {
        return PathBuf::new();
    }

    let name = format!("{kind}-outcomes");
    let prefix = "s3://ferrocene-ci-artifacts/ferrocene/dist";
    let commit = get_closest_upstream_commit(None, &builder.config.git_config(), CiEnv::None)
        .expect(&format!("failed to retrieve git commit for ferrocene.{name}=download-ci"))
        .expect("did not find a commit by merge bot");

    let base = builder.out.join("cache").join("ferrocene").join(&name);
    let extracted_dir = base.join("extracted");
    let tarballs_dir = base.join("tarballs");

    let commit_file = extracted_dir.join(".ferrocene-commit");
    let tarball_file = tarballs_dir.join(&format!("{commit}.tar.xz"));

    if !tarball_file.exists() {
        builder.info(&format!("Downloading {kind} outcomes for commit {commit}"));
        let version = builder.config.artifact_version_part(&commit);
        let url = format!("{prefix}/{commit}/ferrocene-{name}-{version}.tar.xz");
        builder.create_dir(&tarballs_dir);
        builder.config.download_file(
            &url,
            &tarball_file,
            &format!("Could not download the {kind} outcomes."),
        );
    }

    if !commit_file.exists() || builder.read(&commit_file) != commit {
        builder.info(&format!("Extracting {kind} outcomes for commit {commit}"));
        if extracted_dir.exists() {
            builder.remove_dir(&extracted_dir);
        }
        builder.create_dir(&extracted_dir);
        builder.config.unpack(&tarball_file, &extracted_dir, "");
        std::fs::write(&commit_file, commit.as_bytes()).unwrap();
    }

    extracted_dir.join("share").join("ferrocene").join(&name)
}
