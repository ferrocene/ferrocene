// SPDX-License-Identifier: MIT OR Apache-2.0
// SPDX-FileCopyrightText: The Ferrocene Developers

use std::path::Path;
use std::process::Command;

use anyhow::{Context, Error};
use tempfile::NamedTempFile;

use crate::Env;
use crate::config::Config;
use crate::cosign_bundle::RawCosignBundle;
use crate::pinned::Pinned;
use crate::signature_files::SignatureFiles;

pub(crate) fn sign(
    source_dir: &Path,
    output_dir: &Path,
    force: bool,
    env: &Env,
) -> Result<(), Error> {
    let config = Config::load(source_dir)?;
    let pinned = Pinned::generate(env, output_dir)?;
    let mut signature_files = SignatureFiles::load(source_dir, env)?;

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

    // Avoid signing again if the signatures are up to date.
    //
    // Unfortunately this is not a perfect implementation: if there are multiple roles that have to
    // sign the document, and only one role signed it so far, they will have to sign again. This is
    // because we don't know who is signing the document until after the signature is done, and we
    // cannot distinguish the person who signed from who didn't yet.
    let all_signatures_present = config
        .roles
        .keys()
        .all(|role| signature_files.file_exists(&format!("{role}.cosign-bundle")));
    if !force && !regenerate_pinned && all_signatures_present {
        eprintln!();
        eprintln!("The current version of this document has been signed already by all parties.");
        eprintln!("The existing signatures will be reused!");
        eprintln!("Pass --force to regenerate the signatures anyway.");
        eprintln!();
        return Ok(());
    }

    if regenerate_pinned {
        let mut contents = Vec::new();
        contents.extend_from_slice(pinned.toml_comments()?.as_bytes());
        contents.extend_from_slice(toml::to_string_pretty(&pinned)?.as_bytes());

        signature_files.write("pinned.toml", &contents)?;
    }

    let bundle_temp = NamedTempFile::new()?;
    let pinned_temp = signature_files.on_disk_as_tempfile("pinned.toml")?.unwrap();
    let status = Command::new(&env.cosign_binary)
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
