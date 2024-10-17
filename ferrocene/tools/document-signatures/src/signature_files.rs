// SPDX-License-Identifier: MIT OR Apache-2.0
// SPDX-FileCopyrightText: The Ferrocene Developers

//! This module implements an abstraction for reading and writing signature-related files. Behind
//! the scenes, the files are written to an S3 bucket and recorded in the `signature.toml` file,
//! and files are read from the `src/bootstrap`-maintained cache.

use std::collections::BTreeMap;
use std::fs::File;
use std::io::{BufWriter, Seek, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use anyhow::{Context, Error, bail};
use tempfile::NamedTempFile;
use uuid::Uuid;

use crate::{CliOptions, TOML_HEADER_COMMENTS};

pub(crate) struct SignatureFiles<'opts> {
    signature_toml: Signature,
    signature_toml_path: PathBuf,
    options: &'opts CliOptions,
}

impl<'opts> SignatureFiles<'opts> {
    pub(crate) fn load(document: &Path, options: &'opts CliOptions) -> Result<Self, Error> {
        let signature_toml_path = document.join("signature").join("signature.toml");

        let signature_toml = if signature_toml_path.exists() {
            toml::from_slice(&std::fs::read(&signature_toml_path)?)?
        } else {
            Signature { files: BTreeMap::new() }
        };

        Ok(Self { signature_toml, signature_toml_path, options })
    }

    pub(crate) fn read(&self, name: &str) -> Result<Option<Vec<u8>>, Error> {
        // Treat files not mentioned in `signature.toml` as missing.
        let Some(uuid) = self.signature_toml.files.get(name) else {
            return Ok(None);
        };

        Ok(Some(
            std::fs::read(self.options.s3_cache_dir.join(uuid.to_string()))
                // Assume that if a file is in `signature.toml` it must exist in S3.
                .context("this is a bootstrap bug (file is supposed to be cached)")
                .with_context(|| {
                    format!("failed to retrieve signature file {name} (with UUID {uuid})")
                })?,
        ))
    }

    pub(crate) fn on_disk_as_tempfile(&self, name: &str) -> Result<Option<NamedTempFile>, Error> {
        // This function creates temporary files even though the signature files are already on
        // disk due to the cache. This is not the most optimized way to do so, but the API of this
        // function should return a NamedTempFile to support in the future downloading signatures
        // on the fly (without changing the rest of the code).

        // Treat files not mentioned in `signature.toml` as missing.
        let Some(uuid) = self.signature_toml.files.get(name) else {
            return Ok(None);
        };

        let mut cache = File::open(self.options.s3_cache_dir.join(uuid.to_string()))
            .context("this is a bootstrap bug (the file is supposed to be cached)")
            .with_context(|| {
                format!("failed to retrieve signature file {name} (with UUID {uuid})")
            })?;

        let mut tempfile = NamedTempFile::new()?;
        std::io::copy(&mut cache, &mut tempfile)?;
        tempfile.rewind()?;

        Ok(Some(tempfile))
    }

    pub(crate) fn write(&mut self, name: &str, contents: &[u8]) -> Result<(), Error> {
        let Some(s3_bucket) = &self.options.s3_bucket else {
            panic!("uploading signatures is only supported with the s3 backend");
        };
        let uuid = Uuid::new_v4();

        // First off, we upload the file to S3, named after the UUID.
        let mut command = Command::new("aws")
            .args(["s3", "cp", "-"])
            .arg(format!("s3://{s3_bucket}/{uuid}"))
            .stdin(Stdio::piped())
            .spawn()
            .with_context(|| format!("failed to invoke AWS CLI to upload {name}"))?;

        let mut stdin = command.stdin.take().unwrap();
        stdin.write_all(contents)?;
        drop(stdin); // Close stdin

        let result = command
            .wait()
            .with_context(|| format!("failed to wait for AWS CLI completion (to upload {name})"))?;
        if !result.success() {
            bail!("uploading {name} to S3 exited with {result}");
        }

        // Then we write the file in the local cache, to avoid having bootstrap read it from S3
        // the next time it's invoked.
        std::fs::write(self.options.s3_cache_dir.join(uuid.to_string()), contents)?;

        // And finally we update `signature.toml` to record the UUID of the file.
        self.signature_toml.files.insert(name.into(), uuid);
        self.persist()?;

        Ok(())
    }

    fn persist(&self) -> Result<(), Error> {
        if let Some(parent) = self.signature_toml_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let mut output = BufWriter::new(File::create(&self.signature_toml_path)?);
        output.write_all(TOML_HEADER_COMMENTS.as_bytes())?;
        output.write_all(&toml::to_vec(&self.signature_toml)?)?;

        Ok(())
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
struct Signature {
    files: BTreeMap<String, Uuid>,
}
