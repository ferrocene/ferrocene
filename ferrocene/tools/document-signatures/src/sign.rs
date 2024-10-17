// SPDX-License-Identifier: MIT OR Apache-2.0
// SPDX-FileCopyrightText: The Ferrocene Developers

use std::path::Path;
use std::process::Command;

use anyhow::{Context, Error};
use tempfile::NamedTempFile;

use crate::CliOptions;
use crate::config::Config;
use crate::cosign_bundle::RawCosignBundle;
use crate::pinned::Pinned;
use crate::signature_files::SignatureFiles;

pub(crate) fn sign(
    source_dir: &Path,
    output_dir: &Path,
    options: &CliOptions,
) -> Result<(), Error> {
    let config = Config::load(source_dir)?;
    let pinned = Pinned::generate(output_dir)?;
    let mut signature_files = SignatureFiles::load(source_dir, options)?;

    let regenerate_pinned = if let Some(existing_raw) = signature_files.read("pinned.toml")? {
        // The raw contents of pinned.toml are not reproducible, since they intentionally contain
        // random data. If we were to always regenerate the pinned.toml file, old signatures would
        // invalidate as the random data would change. Instead, we only regenerate pinned.toml if
        // the *actual* deserialized data in pinned.toml changed.
        let existing: Pinned =
            toml::from_slice(&existing_raw).context("failed to parse existing pinned.toml")?;
        existing != pinned
    } else {
        true
    };

    if regenerate_pinned {
        let mut contents = Vec::new();
        contents.extend_from_slice(pinned.toml_comments()?.as_bytes());
        contents.extend_from_slice(toml::to_string_pretty(&pinned)?.as_bytes());

        signature_files.write("pinned.toml", &contents)?;
    }

    let bundle_temp = NamedTempFile::new()?;
    let pinned_temp = signature_files.on_disk_as_tempfile("pinned.toml")?.unwrap();
    let status = Command::new(&options.cosign_binary)
        .arg("sign-blob")
        .arg(pinned_temp.path())
        .arg("--bundle")
        .arg(bundle_temp.path())
        .arg("--yes")
        .status()?;
    if !status.success() {
        anyhow::bail!("failed to invoke cosign (exited with status {status})");
    }

    let raw_bundle = RawCosignBundle::load(bundle_temp.path())?;
    let bundle = raw_bundle.parse()?;

    let email = bundle.email()?;
    let Some((role_name, role)) = config.roles.iter().find(|(_, role)| role.email == email) else {
        anyhow::bail!("email {email} has no role in the document's signature config.toml");
    };

    let role_idp = role.idp()?;
    if role_idp.url != bundle.idp()? {
        anyhow::bail!("you must authenticate with {}", role_idp.display_name);
    }

    signature_files
        .write(&format!("{role_name}.cosign-bundle"), &std::fs::read(bundle_temp.path())?)?;

    Ok(())
}
