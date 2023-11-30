// SPDX-License-Identifier: MIT OR Apache-2.0
// SPDX-FileCopyrightText: The Ferrocene Developers

use crate::utils::DisplayList;
use std::ffi::OsString;
use std::fmt::Display;
use std::path::PathBuf;
use std::process::Output;

#[derive(Debug)]
pub(crate) enum Error {
    NoSysroot,
    MissingBinary { directory: PathBuf, name: String },
    WrongBinaryPermissions { path: PathBuf },
    MetadataFetchFailed { path: PathBuf, error: std::io::Error },
    VersionFetchFailed { binary: String, error: CommandError },
    VersionParseFailed { binary: String },
    BinaryVersionMismatch { binary: String, field: String, expected: String, found: String },
    TargetLibraryMissing { target: String, library: String },
    DuplicateTargetLibrary { target: String, library: String },
    TargetLibraryDiscoveryFailed { path: PathBuf, error: std::io::Error },
    LinkerNotFound { targets: Vec<String>, name: String, error: FindBinaryInPathError },
    LinkerVersionFetchFailed { targets: Vec<String>, name: String, error: CommandError },
    LinkerVersionParseFailed { targets: Vec<String>, name: String },
    UnsupportedLinkerVersion { targets: Vec<String>, name: String, expected: String, found: String },
    BundledLinkerMissing { targets: Vec<String> },
    NonUtf8Path { path: PathBuf },
    TemporaryCompilationDirectoryCreationFailed { error: std::io::Error },
    WritingSampleProgramFailed { name: String, dest: PathBuf, error: std::io::Error },
    SampleProgramCompilationFailed { name: String, error: CommandError },
    CompilationArtifactsListingFailed { path: PathBuf, error: std::io::Error },
    MissingCompilationArtifact { name: String, after_compiling: String },
    UnexpectedCompilationArtifact { name: String, after_compiling: String },
}

impl Error {
    pub(crate) fn code(&self) -> u8 {
        match self {
            Error::NoSysroot => 1,
            Error::MissingBinary { .. } => 2,
            Error::WrongBinaryPermissions { .. } => 3,
            Error::MetadataFetchFailed { .. } => 4,
            Error::VersionFetchFailed { .. } => 5,
            Error::VersionParseFailed { .. } => 6,
            Error::BinaryVersionMismatch { .. } => 7,
            Error::TargetLibraryMissing { .. } => 8,
            Error::DuplicateTargetLibrary { .. } => 9,
            Error::TargetLibraryDiscoveryFailed { .. } => 10,
            Error::LinkerNotFound { .. } => 11,
            Error::LinkerVersionFetchFailed { .. } => 12,
            Error::LinkerVersionParseFailed { .. } => 13,
            Error::UnsupportedLinkerVersion { .. } => 14,
            Error::BundledLinkerMissing { .. } => 15,
            Error::NonUtf8Path { .. } => 16,
            Error::TemporaryCompilationDirectoryCreationFailed { .. } => 17,
            Error::WritingSampleProgramFailed { .. } => 18,
            Error::SampleProgramCompilationFailed { .. } => 19,
            Error::CompilationArtifactsListingFailed { .. } => 20,
            Error::MissingCompilationArtifact { .. } => 21,
            Error::UnexpectedCompilationArtifact { .. } => 22,
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::NoSysroot => None,
            Error::MissingBinary { .. } => None,
            Error::WrongBinaryPermissions { .. } => None,
            Error::MetadataFetchFailed { error, .. } => Some(error),
            Error::VersionFetchFailed { error, .. } => Some(error),
            Error::VersionParseFailed { .. } => None,
            Error::BinaryVersionMismatch { .. } => None,
            Error::TargetLibraryMissing { .. } => None,
            Error::DuplicateTargetLibrary { .. } => None,
            Error::TargetLibraryDiscoveryFailed { error, .. } => Some(error),
            Error::LinkerNotFound { error, .. } => Some(error),
            Error::LinkerVersionFetchFailed { error, .. } => Some(error),
            Error::LinkerVersionParseFailed { .. } => None,
            Error::UnsupportedLinkerVersion { .. } => None,
            Error::BundledLinkerMissing { .. } => None,
            Error::NonUtf8Path { .. } => None,
            Error::TemporaryCompilationDirectoryCreationFailed { error } => Some(error),
            Error::WritingSampleProgramFailed { error, .. } => Some(error),
            Error::SampleProgramCompilationFailed { error, .. } => Some(error),
            Error::CompilationArtifactsListingFailed { error, .. } => Some(error),
            Error::MissingCompilationArtifact { .. } => None,
            Error::UnexpectedCompilationArtifact { .. } => None,
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::NoSysroot => {
                write!(f, "could not detect the sysroot of the Ferrocene installation")
            }
            Error::MissingBinary { directory, name } => {
                write!(f, "binary {name} not found (inside {})", directory.display())
            }
            Error::WrongBinaryPermissions { path } => {
                write!(f, "binary {} must be readable and executable by all users", path.display())
            }
            Error::MetadataFetchFailed { path, .. } => {
                write!(f, "failed to fetch the metadata of {}", path.display())
            }
            Error::VersionFetchFailed { binary, .. } => {
                write!(f, "failed to invoke {binary} to fetch its version")
            }
            Error::VersionParseFailed { binary } => {
                write!(f, "failed to parse {binary}'s version output")
            }
            Error::BinaryVersionMismatch { binary, field, expected, found } => {
                write!(f, "expected {field} of {binary} to be {expected}, found {found}")
            }
            Error::TargetLibraryMissing { target, library } => {
                write!(f, "library {library} is missing from target {target}")
            }
            Error::DuplicateTargetLibrary { target, library } => {
                write!(f, "there are conflicting copies of library {library} for target {target}")
            }
            Error::TargetLibraryDiscoveryFailed { path, .. } => {
                write!(f, "failed to access {} while discovering target libraries", path.display())
            }
            Error::LinkerNotFound { targets, name, .. } => {
                write!(
                    f,
                    "linker {name} for target {} not found on the system",
                    DisplayList(targets)
                )
            }
            Error::LinkerVersionFetchFailed { targets, name, .. } => {
                write!(
                    f,
                    "failed to invoke linker {name} to retrieve its version (for target {})",
                    DisplayList(targets)
                )
            }
            Error::LinkerVersionParseFailed { targets, name } => {
                write!(
                    f,
                    "failed to parse the version of linker {name} (for target {})",
                    DisplayList(targets)
                )
            }
            Error::UnsupportedLinkerVersion { targets, name, expected, found } => {
                write!(
                    f,
                    "version {found} of linker {name} is unsupported, only {expected} \
                     is supported (for target {})",
                    DisplayList(targets)
                )
            }
            Error::BundledLinkerMissing { targets } => {
                write!(f, "the bundled linker is missing (for target {})", DisplayList(targets))
            }
            Error::NonUtf8Path { path } => {
                write!(
                    f,
                    "the path {} contains bytes not representable as UTF-8",
                    path.to_string_lossy()
                )
            }
            Error::TemporaryCompilationDirectoryCreationFailed { .. } => {
                write!(f, "failed to create the temporary directory to store compilation artifacts")
            }
            Error::WritingSampleProgramFailed { name, dest, .. } => {
                write!(
                    f,
                    "writing the source of sample program `{name}` to {} failed",
                    dest.display()
                )
            }
            Error::SampleProgramCompilationFailed { name, .. } => {
                write!(f, "compilation of sample program `{name}` failed")
            }
            Error::CompilationArtifactsListingFailed { path, .. } => {
                write!(f, "failed to discover compilation artifacts in {}", path.display())
            }
            Error::MissingCompilationArtifact { name, after_compiling } => {
                write!(
                    f,
                    "missing compilation artifact '{name}' \
                     after compiling sample program `{after_compiling}`"
                )
            }
            Error::UnexpectedCompilationArtifact { name, after_compiling } => {
                write!(
                    f,
                    "unexpected compilation artifact '{name}' \
                     after compiling sample program `{after_compiling}`"
                )
            }
        }
    }
}

#[derive(Debug)]
pub(crate) struct CommandError {
    pub(crate) path: PathBuf,
    pub(crate) args: Vec<OsString>,
    pub(crate) kind: CommandErrorKind,
}

impl std::error::Error for CommandError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.kind {
            CommandErrorKind::StartupFailed { error } => Some(error),
            CommandErrorKind::WaitFailed { error } => Some(error),
            CommandErrorKind::Failure { .. } => None,
            CommandErrorKind::NonUtf8Output => None,
        }
    }
}

impl Display for CommandError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let cli = std::iter::once(self.path.as_os_str())
            .chain(self.args.iter().map(|s| s.as_os_str()))
            .map(|s| s.to_string_lossy())
            .map(|arg| {
                // Quote args with space. This is extremely rudimentary shell quoting, but
                // it should do the trick most of the times.
                if arg.contains(" ") {
                    let arg = arg.replace("\"", "\\\"");
                    format!("\"{arg}\"").into()
                } else {
                    arg
                }
            })
            .collect::<Vec<_>>()
            .join(" ");

        match &self.kind {
            CommandErrorKind::StartupFailed { .. } => {
                write!(f, "failed to invoke `{cli}`")
            }
            CommandErrorKind::WaitFailed { .. } => {
                write!(f, "failed to wait for `{cli}` to finish")
            }
            CommandErrorKind::Failure { output } => {
                write!(f, "invoking `{cli}` failed with {}", output.status)
            }
            CommandErrorKind::NonUtf8Output => {
                write!(f, "invoking `{cli}` returned non-UTF-8 data in its standard output")
            }
        }
    }
}

#[derive(Debug)]
pub(crate) enum CommandErrorKind {
    StartupFailed { error: std::io::Error },
    WaitFailed { error: std::io::Error },
    Failure { output: Output },
    NonUtf8Output,
}

#[derive(Debug)]
pub(crate) enum FindBinaryInPathError {
    NoEnvironmentVariable,
    MissingBinary { name: String },
}

impl std::error::Error for FindBinaryInPathError {}

impl Display for FindBinaryInPathError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FindBinaryInPathError::NoEnvironmentVariable => {
                write!(f, "the PATH environment variable is not set")
            }
            FindBinaryInPathError::MissingBinary { name } => {
                write!(f, "binary {name} is not present in the system PATH")
            }
        }
    }
}
