// SPDX-License-Identifier: MIT OR Apache-2.0
// SPDX-FileCopyrightText: The Ferrocene Developers

//! This module contains the implementation of `./x sign`.
//!
//! All the actual steps are automatically generated, and they all invoke the
//! [SignDocument] step behind the scenes to actually perform the signature of
//! the documentation.

mod cachesignaturefiles;
mod cosignbinary;
mod makro;
mod signdocument;

use std::path::Path;
use std::process::Command;

use crate::builder::{Builder, RunConfig, ShouldRun, Step};
use crate::core::build_steps::tool::Tool;
use crate::core::config::TargetSelection;
use crate::ferrocene::doc::{SphinxMode, WithSource};

pub(crate) use self::cachesignaturefiles::CacheSignatureFiles;
use self::{cosignbinary::CosignBinary, makro::documents, signdocument::SignDocument};

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

    let mut cmd = Command::new(tool);
    cmd.env("DOCUMENT_SIGNATURES_COSIGN_BINARY", &cosign);
    cmd.env("DOCUMENT_SIGNATURES_S3_BUCKET", "ferrocene-document-signatures");
    cmd.env("DOCUMENT_SIGNATURES_S3_CACHE_DIR", &cache_dir);
    if let Some(profile) = &builder.config.ferrocene_aws_profile {
        cmd.env("AWS_PROFILE", profile);
    }
    cmd
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
