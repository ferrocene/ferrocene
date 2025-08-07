// SPDX-License-Identifier: MIT OR Apache-2.0
// SPDX-FileCopyrightText: The Ferrocene Developers

use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::marker::PhantomData;
use std::path::{Path, PathBuf};

use tar::Archive;
use xz2::read::XzDecoder;

use crate::core::builder::{Builder, ShouldRun, Step};
use crate::core::config::FerroceneDocumentSignatures;
use crate::ferrocene::doc::IsSphinxBook;
use crate::t;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct CacheSignatureFiles<B: Step + IsSphinxBook> {
    marker: PhantomData<B>,
}

impl<B: Step + IsSphinxBook> CacheSignatureFiles<B> {
    pub(crate) fn new() -> Self {
        Self { marker: PhantomData }
    }
}

impl<B: Step + IsSphinxBook> Step for CacheSignatureFiles<B> {
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

        let signature_toml_path =
            builder.src.join(B::SOURCE).join("signature").join("signature.toml");
        let signature_toml: SignatureToml = match std::fs::read(&signature_toml_path) {
            Ok(contents) => t!(toml::from_slice(&contents)),
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => return cache_dir,
            Err(err) => panic!("failed to read {}: {err}", signature_toml_path.display()),
        };

        let fetcher: Box<dyn SignatureFilesFetcher> =
            match &builder.config.ferrocene_document_signatures {
                FerroceneDocumentSignatures::Disabled => return cache_dir,
                FerroceneDocumentSignatures::S3 { bucket } => {
                    Box::new(S3FilesFetcher { bucket: bucket.clone() })
                }
                FerroceneDocumentSignatures::DocsTarball { tarball } => {
                    Box::new(DocsTarballFilesFetcher::<B> {
                        path: tarball.clone(),
                        phantom: PhantomData,
                    })
                }
            };
        for (name, uuid) in signature_toml.files.into_iter() {
            let cached_file = &cache_dir.join(&uuid);
            if cached_file.exists() {
                continue;
            }
            fetcher.fetch(builder, &uuid, &name, &cached_file);
        }

        cache_dir
    }
}

trait SignatureFilesFetcher {
    fn fetch(&self, builder: &Builder<'_>, uuid: &str, name: &str, dest: &Path);
}

struct S3FilesFetcher {
    bucket: String,
}

impl SignatureFilesFetcher for S3FilesFetcher {
    fn fetch(&self, builder: &Builder<'_>, uuid: &str, name: &str, dest: &Path) {
        builder.config.download_file(
            &format!("s3://{}/{uuid}", self.bucket),
            dest,
            // \u{20} is a space.
            &format!(
                "Failed to download signature file {uuid}, corresponding to {name}.\n\
                     \n\
                     If you don't have access to private signature files, you must disable \n\
                     digital signature support to continue executing this command. You can \n\
                     do so by adding this to your `config.toml`:\n\
                     \n\
                     \u{20}  [ferrocene]\n\
                     \u{20}  document-signatures = \"disabled\""
            ),
        );
    }
}

struct DocsTarballFilesFetcher<P: IsSphinxBook> {
    path: PathBuf,
    phantom: PhantomData<P>,
}

impl<P: IsSphinxBook> SignatureFilesFetcher for DocsTarballFilesFetcher<P> {
    fn fetch(&self, _builder: &Builder<'_>, _uuid: &str, name: &str, dest: &Path) {
        let expected_path = format!("share/doc/ferrocene/html/{}/signature/{name}", P::DEST);
        match self.fetch_inner(&expected_path, dest) {
            Ok(()) => {}
            Err(err) => {
                panic!("Failed to extract {expected_path} from {}: {err}", self.path.display())
            }
        }
    }
}

impl<P: IsSphinxBook> DocsTarballFilesFetcher<P> {
    fn fetch_inner(&self, path: &str, dest: &Path) -> Result<(), Box<dyn Error>> {
        let mut tarball = Archive::new(XzDecoder::new(BufReader::new(File::open(&self.path)?)));
        for entry in tarball.entries()? {
            let mut entry = entry?;
            if entry.path()?.to_str() == Some(path) {
                entry.unpack(dest)?;
                return Ok(());
            }
        }
        Err(Box::<dyn Error>::from("file not present in the archive"))
    }
}
