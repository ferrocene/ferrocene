// SPDX-License-Identifier: MIT OR Apache-2.0
// SPDX-FileCopyrightText: The Ferrocene Developers

// This module contains the implementation of `./x sign`. All the actual steps are automatically
// generated, and they all invoke the SignDocument step behind the scenes to actually perform the
// signature of the documentation.

mod cosign;
pub(crate) mod signature_files;

use crate::builder::{Builder, RunConfig, ShouldRun, Step};
use crate::core::build_steps::tool::Tool;
use crate::core::config::{self, TargetSelection};
use crate::ferrocene::doc::{IsSphinxBook, SphinxMode};
use crate::ferrocene::sign::signature_files::CacheSignatureFiles;
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct SignDocument<S: Step<Output = PathBuf> + IsSphinxBook> {
    target: TargetSelection,
    document: S,
}

impl<S: Step<Output = PathBuf> + IsSphinxBook> Step for SignDocument<S> {
    type Output = ();
    const DEFAULT: bool = false;
    const ONLY_HOSTS: bool = true;

    fn should_run(run: ShouldRun<'_>) -> ShouldRun<'_> {
        run.never()
    }

    fn run(self, builder: &Builder<'_>) {
        error_when_signatures_are_ignored(builder, "sign a document");

        let document = builder.ensure(self.document);
        builder.run(
            document_signatures_cmd::<S>(builder)
                .arg("sign")
                .arg(builder.src.join(S::SOURCE))
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
            f: impl Fn(Command, &Path, &Path),
        ) {
            $({
                let source_dir = builder.src.join(crate::ferrocene::doc::$name::SOURCE);
                if condition(&source_dir) {
                    let output_dir = builder.ensure(crate::ferrocene::doc::$name {
                        mode: SphinxMode::Html,
                        target,
                        fresh_build: false,
                    });
                    let cmd = document_signatures_cmd::<crate::ferrocene::doc::$name>(builder);
                    f(cmd, &source_dir, &output_dir);
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

pub(super) fn document_signatures_cmd<B: Step + IsSphinxBook>(builder: &Builder<'_>) -> Command {
    let cosign = builder.ensure(cosign::CosignBinary);
    let cache_dir = builder.ensure(CacheSignatureFiles::<B>::new());
    let tool = builder.tool_exe(Tool::FerroceneDocumentSignatures);

    let mut cmd = Command::new(&tool);
    cmd.env("DOCUMENT_SIGNATURES_COSIGN_BINARY", &cosign);
    cmd.env("DOCUMENT_SIGNATURES_S3_CACHE_DIR", &cache_dir);
    match &builder.config.ferrocene_document_signatures {
        config::FerroceneDocumentSignatures::Disabled => {}
        config::FerroceneDocumentSignatures::DocsTarball { .. } => {}
        config::FerroceneDocumentSignatures::S3 { bucket } => {
            cmd.env("DOCUMENT_SIGNATURES_S3_BUCKET", bucket);
        }
    }
    if let Some(profile) = &builder.config.ferrocene_aws_profile {
        cmd.env("AWS_PROFILE", profile);
    }
    cmd
}

pub(crate) fn error_when_signatures_are_ignored(builder: &Builder<'_>, action: &str) {
    if let config::FerroceneDocumentSignatures::Disabled =
        &builder.config.ferrocene_document_signatures
    {
        eprintln!("You're trying to {action} when document signatures are ignored.");
        eprintln!();
        eprintln!("If you are a Ferrous Systems employee or contractor with access to AWS");
        eprintln!("and you need to enable signatures, **REMOVE** the following setting");
        eprintln!("from your config.toml:");
        eprintln!();
        eprintln!("   [ferrocene]");
        eprintln!("   document-signatures = \"disabled\"");
        eprintln!();
        panic!("document signatures are ignored");
    }
}
