// SPDX-License-Identifier: MIT OR Apache-2.0
// SPDX-FileCopyrightText: The Ferrocene Developers

use std::collections::HashMap;
use std::path::PathBuf;

use crate::builder::{Builder, ShouldRun, Step};
use crate::t;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct CacheSignatureFiles {
    pub(crate) source_dir: PathBuf,
}

impl Step for CacheSignatureFiles {
    type Output = PathBuf;

    fn should_run(run: ShouldRun<'_>) -> ShouldRun<'_> {
        run.never()
    }

    fn run(self, builder: &Builder<'_>) -> Self::Output {
        #[derive(serde_derive::Deserialize)]
        struct SignatureToml {
            files: HashMap<String, String>,
        }

        let cache_dir = builder.out.join("cache").join("ferrocene-document-signatures");
        if builder.config.dry_run() {
            return cache_dir;
        }
        if !cache_dir.exists() {
            builder.create_dir(&cache_dir);
        }

        let signature_toml_path = self.source_dir.join("signature").join("signature.toml");
        let signature_toml: SignatureToml = match std::fs::read(&signature_toml_path) {
            Ok(contents) => t!(toml::from_slice(&contents)),
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => return cache_dir,
            Err(err) => panic!("failed to read {}: {err}", signature_toml_path.display()),
        };

        for (name, uuid) in signature_toml.files.into_iter() {
            let cached_file = &cache_dir.join(&uuid);
            if cached_file.exists() {
                continue;
            }
            builder.config.download_file(
                &format!("s3://{}/{uuid}", builder.config.ferrocene_document_signatures_s3_bucket),
                &cached_file,
                // \u{20} is a space.
                &format!(
                    "Failed to download signature file {uuid}, corresponding to {name}.\n\
                     \n\
                     If you don't have access to private signature files, you must disable \n\
                     digital signature support to continue executing this command. You can \n\
                     do so by adding this to your `config.toml`:\n\
                     \n\
                     \u{20}  [ferrocene]\n\
                     \u{20}  ignore-document-signatures = true"
                ),
            );
        }

        cache_dir
    }
}
