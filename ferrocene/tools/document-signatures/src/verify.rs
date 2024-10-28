// SPDX-License-Identifier: MIT OR Apache-2.0
// SPDX-FileCopyrightText: The Ferrocene Developers

use std::io::Read;
use std::path::Path;
use std::process::Command;

use anyhow::{Context, Error, anyhow};

use crate::CliOptions;
use crate::config::Config;
use crate::pinned::Pinned;
use crate::signature_files::SignatureFiles;

pub(crate) fn verify(
    source_dir: &Path,
    output_dir: &Path,
    options: &CliOptions,
) -> Result<(), Error> {
    let signature_files = SignatureFiles::load(source_dir, options)?;

    let pinned_toml = if let Some(mut file) = signature_files.on_disk_as_tempfile("pinned.toml")? {
        let mut contents = Vec::new();
        file.read_to_end(&mut contents)?;

        let existing: Pinned = toml::from_slice(&contents)?;
        let expected = Pinned::generate(output_dir)?;

        if existing != expected {
            if existing.document_id != expected.document_id {
                eprintln!("existing document id: {}", existing.document_id);
                eprintln!("expected document id: {}", expected.document_id);
            }
            if existing.tarball_sha256 != expected.tarball_sha256 {
                eprintln!("existing tarball sha256: {}", existing.tarball_sha256);
                eprintln!("expected tarball sha256: {}", expected.tarball_sha256);
            }
            anyhow::bail!("pinned documentation file outdated");
        }

        file
    } else {
        // The document was not signed.
        return Ok(());
    };

    let config = Config::load(source_dir)?;
    for (role_name, role) in config.roles.iter() {
        let bundle = signature_files
            .on_disk_as_tempfile(&format!("{role_name}.cosign-bundle"))
            .with_context(|| format!("failed to read signature for role {role_name}"))?
            .ok_or_else(|| anyhow!("missing signature file for role {role_name}"))?;

        eprintln!("checking role {role_name}");
        let status = Command::new(&options.cosign_binary)
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
