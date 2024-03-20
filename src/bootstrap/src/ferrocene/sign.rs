// SPDX-License-Identifier: MIT OR Apache-2.0
// SPDX-FileCopyrightText: The Ferrocene Developers

// This module contains the implementation of `./x sign`. All the actual steps are automatically
// generated, and they all invoke the SignDocument step behind the scenes to actually perform the
// signature of the documentation.

use crate::builder::{Builder, RunConfig, ShouldRun, Step};
use crate::core::build_steps::tool::Tool;
use crate::core::config::TargetSelection;
use crate::ferrocene::doc::{SphinxMode, WithSource};
use crate::t;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::process::Command;

// Latest version and checksums available at: https://github.com/sigstore/cosign/releases
const COSIGN_VERSION: &str = "2.2.3";
const COSIGN_ARTIFACTS: &[CosignArtifact] = &[
    CosignArtifact {
        target: "x86_64-unknown-linux-gnu",
        name: "cosign-linux-amd64",
        sha256: "f669f41176cb1d58bb6a3fdb06e24861540cfdb5a571b4ec5eb2218b0df5d304",
    },
    CosignArtifact {
        target: "x86_64-pc-windows-msvc",
        name: "cosign-windows-amd64.exe",
        sha256: "f7f272d56c580b0ec96f59bfe9f88ec5f42b6e195df009ce3417428e0e0dead1",
    },
    CosignArtifact {
        target: "x86_64-apple-darwin",
        name: "cosign-darwin-amd64",
        sha256: "2429f4b027fc311a6324e9db6fb3a937d559dc61de906a1c2d0d1e0671685e4c",
    },
    CosignArtifact {
        target: "aarch64-apple-darwin",
        name: "cosign-darwin-arm64",
        sha256: "3d95ab46d4c4cc55e6465758c238dc03f830cc8a1fc38bc7a33bc203e0fb2c3b",
    },
];

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct SignDocument<S: Step<Output = PathBuf> + WithSource> {
    target: TargetSelection,
    document: S,
}

impl<S: Step<Output = PathBuf> + WithSource> Step for SignDocument<S> {
    type Output = ();
    const DEFAULT: bool = false;
    const ONLY_HOSTS: bool = true;

    fn should_run(run: ShouldRun<'_>) -> ShouldRun<'_> {
        run.never()
    }

    fn run(self, builder: &Builder<'_>) {
        error_when_signatures_are_ignored(builder, "sign a document");

        let document = builder.ensure(self.document);
        let source_dir = builder.src.join(S::SOURCE);
        builder.run(
            document_signatures_cmd(builder, &source_dir)
                .arg("sign")
                .arg(&source_dir)
                .arg(&document),
        );
    }
}

macro_rules! documents {
    ($($name:ident),*$(,)?) => {
        $(
            #[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
            pub(crate) struct $name {
                target: TargetSelection,
            }

            impl Step for $name {
                type Output = ();
                const DEFAULT: bool = false;
                const ONLY_HOSTS: bool = true;

                fn should_run(run: ShouldRun<'_>) -> ShouldRun<'_> {
                    crate::ferrocene::doc::$name::should_run(run)
                }

                fn make_run(run: RunConfig<'_>) {
                    run.builder.ensure(Self { target: run.target });
                }

                fn run(self, builder: &Builder<'_>) {
                    builder.ensure(SignDocument {
                        target: self.target,
                        document: crate::ferrocene::doc::$name {
                            mode: SphinxMode::Html,
                            target: self.target,
                            // Ensure there are no leftover artifacts from a previous incremental
                            // build when generating the signature.
                            fresh_build: true,
                        },
                    });
                }
            }
        )*

        pub(super) fn for_each_signable_document(
            builder: &Builder<'_>,
            target: TargetSelection,
            condition: impl Fn(&Path) -> bool,
            f: impl Fn(&Path, &Path),
        ) {
            $({
                let source_dir = builder.src.join(crate::ferrocene::doc::$name::SOURCE);
                if condition(&source_dir) {
                    let output_dir = builder.ensure(crate::ferrocene::doc::$name {
                        mode: SphinxMode::Html,
                        target,
                        fresh_build: false,
                    });
                    f(&source_dir, &output_dir);
                }
            })*
        }
    };
}

// Also remember to update the describe! macro in src/bootstrap/builder.rs
documents![
    // Qualification Documents
    DocumentList,
    EvaluationPlan,
    EvaluationReport,
    QualificationPlan,
    QualificationReport,
    SafetyManual,
    // QMS Documents
    InternalProcedures,
];

pub(super) fn document_signatures_cmd(builder: &Builder<'_>, source_dir: &Path) -> Command {
    let cosign = builder.ensure(CosignBinary);
    let cache_dir = builder.ensure(CacheSignatureFiles { source_dir: source_dir.into() });
    let tool = builder.tool_exe(Tool::FerroceneDocumentSignatures);

    let mut cmd = Command::new(&tool);
    cmd.env("DOCUMENT_SIGNATURES_COSIGN_BINARY", &cosign);
    cmd.env("DOCUMENT_SIGNATURES_S3_BUCKET", "ferrocene-document-signatures");
    cmd.env("DOCUMENT_SIGNATURES_S3_CACHE_DIR", &cache_dir);
    if let Some(profile) = &builder.config.ferrocene_aws_profile {
        cmd.env("AWS_PROFILE", profile);
    }
    cmd
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct CosignBinary;

impl Step for CosignBinary {
    type Output = PathBuf;
    const DEFAULT: bool = false;
    const ONLY_HOSTS: bool = true;

    fn should_run(run: ShouldRun<'_>) -> ShouldRun<'_> {
        run.never()
    }

    fn run(self, builder: &Builder<'_>) -> PathBuf {
        if builder.config.dry_run() {
            return PathBuf::new();
        }

        let mut artifact = None;
        for candidate in COSIGN_ARTIFACTS {
            if candidate.target == &*builder.config.build.triple {
                artifact = Some(candidate);
                break;
            }
        }
        let Some(artifact) = artifact else {
            eprintln!();
            eprintln!("error: unsupported platform for cosign: {}", builder.config.build);
            eprintln!("note:  add support for it in src/bootstrap/ferrocene/sign.rs");
            eprintln!();
            panic!("could not download cosign");
        };

        let dest = builder
            .out
            .join(builder.config.build.triple)
            .join("ferrocene")
            .join(format!("cosign-{COSIGN_VERSION}"));
        if let Some(parent) = dest.parent() {
            builder.create_dir(parent);
        }

        let url = format!(
            "https://github.com/sigstore/cosign/releases/download/v{}/{}",
            COSIGN_VERSION, artifact.name
        );

        if !dest.exists() {
            builder.config.download_file(&url, &dest, "test");
        }
        if !builder.config.verify(&dest, artifact.sha256) && !builder.config.dry_run() {
            panic!("invalid cosign downloaded");
        }

        #[cfg(unix)]
        if !builder.config.dry_run() {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = std::fs::metadata(&dest).unwrap().permissions();
            perms.set_mode(0o755);
            std::fs::set_permissions(&dest, perms).unwrap();
        }

        dest
    }
}

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

pub(crate) fn error_when_signatures_are_ignored(builder: &Builder<'_>, action: &str) {
    if builder.config.ferrocene_ignore_document_signatures {
        eprintln!("You're trying to {action} when document signatures are ignored.");
        eprintln!();
        eprintln!("If you are a Ferrous Systems employee or contractor with access to AWS");
        eprintln!("and you need to enable signatures, **REMOVE** the following setting");
        eprintln!("from your config.toml:");
        eprintln!();
        eprintln!("   [ferrocene]");
        eprintln!("   ignore-document-signatures = true");
        eprintln!();
        panic!("document signatures are ignored");
    }
}

struct CosignArtifact {
    target: &'static str,
    name: &'static str,
    sha256: &'static str,
}
