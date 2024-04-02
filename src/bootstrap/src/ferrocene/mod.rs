// SPDX-License-Identifier: MIT OR Apache-2.0
// SPDX-FileCopyrightText: The Ferrocene Developers

pub(crate) mod code_coverage;
pub(crate) mod dist;
pub(crate) mod doc;
pub(crate) mod partners;
pub(crate) mod run;
pub(crate) mod sign;
pub(crate) mod test;
pub(crate) mod test_outcomes;
pub(crate) mod tool;

use crate::builder::Builder;
use crate::core::config::{Config, TargetSelection};
use crate::t;
use std::collections::HashMap;
use std::path::Path;
use std::process::Command;

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

    #[allow(deprecated)]
    let success = config
        .try_run(Command::new("aws").args(["s3", "cp"]).args(profile_flags).arg(url).arg(tempfile));

    if success.is_err() {
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
        .cloned()
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
