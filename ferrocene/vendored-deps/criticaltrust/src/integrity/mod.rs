//! High-level interface to verify the integrity of archives and installations.

mod detect_manifest;
mod verifier;

pub use verifier::{IntegrityVerifier, VerifiedPackage};

/// Integrity error detected by [`IntegrityVerifier`].
#[derive(Debug, thiserror::Error)]
pub enum IntegrityError {
    #[error("failed to deserialize the package manifest at {path}")]
    PackageManifestDeserialization {
        path: String,
        #[source]
        inner: serde_json::Error,
    },
    #[error("failed to verify the package manifest at {path}")]
    PackageManifestVerification {
        path: String,
        #[source]
        inner: crate::Error,
    },
    #[error("wrong POSIX permissions for {path} (expected: {expected:o}, found {found:o})")]
    WrongPosixPermissions {
        path: String,
        expected: u32,
        found: u32,
    },
    #[error("wrong checksum for {path}")]
    WrongChecksum { path: String },
    #[error("the product name of {path} is not {expected} (the file path is wrong)")]
    WrongProductName { path: String, expected: String },
    #[error("the package name of {path} is not {expected} (the file path is wrong)")]
    WrongPackageName { path: String, expected: String },
    #[error("no package manifest found")]
    NoPackageManifestFound,
    #[error("expected file {path} is not present")]
    MissingFile { path: String },
    #[error("unexpected file {path} is present")]
    UnexpectedFile { path: String },
    #[error("unexpected file {path} in prefix managed by criticalup ({prefix})")]
    UnexpectedFileInManagedPrefix { path: String, prefix: String },
    #[error("file {path} is referenced by multiple package manifests")]
    FileReferencedByMultipleManifests { path: String },
    #[error("file {path} was loaded multiple times")]
    FileLoadedMultipleTimes { path: String },
}
