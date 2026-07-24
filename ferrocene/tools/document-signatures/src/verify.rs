// SPDX-License-Identifier: MIT OR Apache-2.0
// SPDX-FileCopyrightText: The Ferrocene Developers

use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::Command;

use anyhow::{Context, Error, Result};

use crate::Env;
use crate::config::Config;
use crate::pinned::Pinned;
use crate::signature_files::SignatureFiles;

fn report_bad_signature(output_dir: &Path, existing: &Pinned, expected: &Pinned, files: &SignatureFiles<'_>) -> Result<()> {
    eprintln!("Signature incorrect: {}", output_dir.display());
    if existing.document_id != expected.document_id {
        eprintln!("existing document id: {}", existing.document_id);
        eprintln!("expected document id: {}", expected.document_id);
    }
    if existing.tarball_sha256 != expected.tarball_sha256 {
        eprintln!("existing tarball sha256: {}", existing.tarball_sha256);
        eprintln!("expected tarball sha256: {}", expected.tarball_sha256);
    }

    // Print a diff that explains why this broke.
    if let Some(expected_tarfile) = files.on_disk_as_tempfile("stable-archive.tar.gz")? {
        let diff_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("diff.py");
        Command::new(diff_path).arg("local").arg(output_dir.file_name().unwrap()).arg(expected_tarfile.path()).status()?;
    } else {
        eprintln!("no tarball was uploaded for this signature, cannot generate a diff");
    }

    Ok(())
}

pub(crate) fn verify(source_dir: &Path, output_dir: &Path, env: &Env) -> Result<(), Error> {
    let signature_files = SignatureFiles::load(source_dir, env).context(format!("failed to load signature for {source_dir:?}"))?;

    let pinned_toml = if let Some(mut file) = signature_files.on_disk_as_tempfile("pinned.toml")? {
        let mut contents = Vec::new();
        file.read_to_end(&mut contents)?;

        let existing: Pinned = toml::from_slice(&contents)?;
        let (expected, _) = Pinned::generate(env, output_dir).context("failed to generate pinned tarball")?;

        if existing != expected {
            if let Err(e) = report_bad_signature(output_dir, &existing, &expected, &signature_files) {
                eprintln!("failed to generate diff: {e:#}");
            }
            anyhow::bail!("pinned documentation file outdated");
        } else {
            eprintln!("Signature correct: {}", output_dir.display());
        }

        file
    } else {
        // The document was not signed.
        eprintln!("WARNING: {} is not signed", output_dir.display());
        return Ok(());
    };

    let config = Config::load(source_dir)?;
    for (role_name, role) in config.roles.iter() {
        let maybe_bundle = signature_files
            .on_disk_as_tempfile(&format!("{role_name}.cosign-bundle"))
            .with_context(|| format!("failed to read signature for role {role_name}"))?;

        let bundle = match maybe_bundle {
            Some(present) => present,
            None if env.allow_dev_signing => {
                eprintln!("warning: missing signature from role {role_name}!");
                eprintln!(
                    "ignoring since this is a dev build, but these documents cannot be used in production"
                );
                continue;
            }
            None => anyhow::bail!("missing signature file for role {role_name}"),
        };

        eprintln!("checking role {role_name}");
        let status = Command::new(&env.cosign_binary)
            .arg("verify-blob")
            .arg(pinned_toml.path())
            .arg("--bundle")
            .arg(bundle.path())
            .args(["--certificate-identity", &role.email])
            .args(["--certificate-oidc-issuer", role.idp()?.url])
            .status()?;
        if !status.success() {
            anyhow::bail!("failed to verify signature for role {role_name} (exited with {status})");
        }
    }

    Ok(())
}
