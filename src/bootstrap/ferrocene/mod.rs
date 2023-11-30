// SPDX-License-Identifier: MIT OR Apache-2.0
// SPDX-FileCopyrightText: The Ferrocene Developers

pub(crate) mod dist;
pub(crate) mod doc;
pub(crate) mod run;
pub(crate) mod sign;
pub(crate) mod test;
pub(crate) mod tool;

use crate::builder::Builder;
use crate::config::{Config, TargetSelection};
use crate::util::t;
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

    let success = config
        .try_run(Command::new("aws").args(["s3", "cp"]).args(profile_flags).arg(url).arg(tempfile));

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

pub(crate) fn ignored_tests_for_suite(
    builder: &Builder<'_>,
    target: TargetSelection,
    suite: &str,
) -> Vec<String> {
    #[derive(serde::Deserialize)]
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
