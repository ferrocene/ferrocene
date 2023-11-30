// SPDX-License-Identifier: MIT OR Apache-2.0
// SPDX-FileCopyrightText: The Ferrocene Developers

use crate::TOML_HEADER_COMMENTS;
use anyhow::{Context, Error};
use sha2::{Digest, Sha256};
use std::path::Path;
use std::process::{Command, Stdio};

const TAR_REPRODUCIBILITY_FLAGS: &[&str] = &[
    // Use a consistent ordering for the files in the archive.
    "--sort=name",
    // Use a pinned date for the modification time, otherwise touching a file would change the hash
    // of the archive.
    "--mtime=2020-01-01T00:00:00Z",
    // Tarballs include the UNIX owner and groups, which change between systems. Hardcode them to
    // "0", and avoid looking up the corresponding name for ID 0.
    "--owner=0",
    "--group=0",
    "--numeric-owner",
    // Make all files readable by everyone and writable by the owner, preserving the executable
    // bit. This is needed otherwise systems with non-reproducible umasks will generate different
    // tarballs.
    "--mode=u+rw,go+r,go-w",
    // The default format includes non-deterministic bits. Use the GNU format, which omits them.
    "--format=gnu",
    // The generated documentation includes a directory called "signature" that contains data about
    // the signature being generated right now. We exclude it from the hash as we can't know the
    // signature contents in advance.
    "--anchored",
    "--exclude=./signature",
];

#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub(crate) struct Pinned {
    pub(crate) document_id: String,
    pub(crate) tarball_sha256: String,
}

impl Pinned {
    pub(crate) fn generate(output_dir: &Path) -> Result<Self, Error> {
        let document_id = std::fs::read_to_string(output_dir.join("document-id.txt"))
            .context("failed to read document-id.txt from the output directory")?
            .trim()
            .to_string();

        let mut tar = Command::new("tar")
            .args(TAR_REPRODUCIBILITY_FLAGS)
            .arg("-C")
            .arg(output_dir)
            .args(&["-c", "."])
            .stdout(Stdio::piped())
            .spawn()?;

        let mut hasher = Sha256::new();
        std::io::copy(&mut tar.stdout.take().unwrap(), &mut hasher)?;
        let tarball_sha256 = hex::encode(hasher.finalize());

        let status = tar.wait()?;
        if !status.success() {
            anyhow::bail!("failed to invoke tar to create content tarball");
        }

        Ok(Self { document_id, tarball_sha256 })
    }

    pub(crate) fn toml_comments(&self) -> Result<String, Error> {
        let mut comments = String::new();
        comments.push_str(TOML_HEADER_COMMENTS);
        comments.push_str("# The tarball-sha256 field can be generated with:\n");

        comments.push_str("# tar -C $path ");
        for flag in TAR_REPRODUCIBILITY_FLAGS {
            comments.push_str(flag);
            comments.push(' ');
        }
        comments.push_str("-c . | sha256sum\n");
        comments.push('\n');

        comments.push_str("# Random bytes are included in the comments to prevent the file from\n");
        comments.push_str("# being reproducible. Otherwise, it would be easy to retrieve the\n");
        comments.push_str("# proprietary signatures from the rekor transparency log.\n");
        comments.push_str("# RANDOM DATA: ");
        self.generate_random_data(&mut comments)?;
        comments.push('\n');
        comments.push('\n');

        Ok(comments)
    }

    fn generate_random_data(&self, into: &mut String) -> Result<(), Error> {
        let mut random_data = [0; 64];
        getrandom::getrandom(&mut random_data)?;
        base64::encode_config_buf(&random_data, base64::STANDARD_NO_PAD, into);
        Ok(())
    }
}
