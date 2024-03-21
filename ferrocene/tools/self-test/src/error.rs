// SPDX-License-Identifier: MIT OR Apache-2.0
// SPDX-FileCopyrightText: The Ferrocene Developers

use std::ffi::OsString;
use std::fmt::Display;
use std::path::PathBuf;
use std::process::Output;

use thiserror::Error as ThisError;

#[derive(Debug, ThisError)]
pub(crate) enum Error {
    #[error("could not detect the sysroot of the Ferrocene installation")]
    NoSysroot,
    #[error("binary {name} not found (inside {})", directory.display())]
    MissingBinary { directory: PathBuf, name: String },
    #[error("binary {} must be readable and executable by all users", path.display())]
    WrongBinaryPermissions { path: PathBuf },
    #[error("failed to fetch the metadata of {}", path.display())]
    MetadataFetchFailed {
        path: PathBuf,
        #[source]
        error: std::io::Error,
    },
    #[error("failed to invoke {binary} to fetch its version")]
    VersionFetchFailed {
        binary: String,
        #[source]
        error: Box<CommandError>,
    },
    #[error("failed to parse {binary}'s version output")]
    VersionParseFailed { binary: String },
    #[error("expected {field} of {binary} to be {expected}, found {found}")]
    BinaryVersionMismatch { binary: String, field: String, expected: String, found: String },
    #[error("library {library} is missing from target {target}")]
    TargetLibraryMissing { target: String, library: String },
    #[error("there are conflicting copies of library {library} for target {target}")]
    DuplicateTargetLibrary { target: String, library: String },
    #[error("failed to access {} while discovering target libraries", path.display())]
    TargetLibraryDiscoveryFailed {
        path: PathBuf,
        #[source]
        error: std::io::Error,
    },
    #[error("C compiler {name} not found on the system")]
    CCompilerNotFound {
        name: String,
        #[source]
        error: FindBinaryInPathError,
    },
    #[error("the bundled linker is missing")]
    BundledLinkerMissing,
    #[error("the path {} contains bytes not representable as UTF-8", path.to_string_lossy())]
    NonUtf8Path { path: PathBuf },
    #[error("failed to create the temporary directory to store compilation artifacts")]
    TemporaryCompilationDirectoryCreationFailed {
        #[source]
        error: std::io::Error,
    },
    #[error("writing the source of sample program `{name}` to {} failed", dest.display())]
    WritingSampleProgramFailed {
        name: String,
        dest: PathBuf,
        #[source]
        error: std::io::Error,
    },
    #[error("compilation of sample program `{name}` failed")]
    SampleProgramCompilationFailed {
        name: String,
        #[source]
        error: Box<CommandError>,
    },
    #[error("failed to discover compilation artifacts in {}", path.display())]
    CompilationArtifactsListingFailed {
        path: PathBuf,
        #[source]
        error: std::io::Error,
    },
    #[error(
        "missing compilation artifact '{name}' after compiling sample program `{after_compiling}`"
    )]
    MissingCompilationArtifact { name: String, after_compiling: String },
    #[error(
        "unexpected compilation artifact '{name}' after compiling sample program `{after_compiling}`"
    )]
    UnexpectedCompilationArtifact { name: String, after_compiling: String },
    #[error("unable to find LLD-compatible C compiler for `{target}`")]
    SuitableCCompilerNotFound { target: String },
    #[error("Unable to analyse linker arguments for `{target}` (kind={kind:?})")]
    WrongLinkerArgs { target: String, kind: LinkerArgsErrorKind },
    #[error("unable to execute sample program {name}")]
    RunningSampleProgramFailed {
        name: String,
        #[source]
        error: std::io::Error,
    },
    #[error(
        "sample program {name} should have produced {expected:?}, actually produced {found:?}"
    )]
    SampleProgramOutputWrong { name: String, expected: Vec<u8>, found: Vec<u8> },
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
            Error::CCompilerNotFound { .. } => 11,
            Error::BundledLinkerMissing => 15,
            Error::NonUtf8Path { .. } => 16,
            Error::TemporaryCompilationDirectoryCreationFailed { .. } => 17,
            Error::WritingSampleProgramFailed { .. } => 18,
            Error::SampleProgramCompilationFailed { .. } => 19,
            Error::CompilationArtifactsListingFailed { .. } => 20,
            Error::MissingCompilationArtifact { .. } => 21,
            Error::UnexpectedCompilationArtifact { .. } => 22,
            Error::SuitableCCompilerNotFound { .. } => 23,
            Error::WrongLinkerArgs { .. } => 24,
            Error::RunningSampleProgramFailed { .. } => 25,
            Error::SampleProgramOutputWrong { .. } => 26,
        }
    }

    pub(crate) fn sample_program_compilation_failed(name: &str, error: CommandError) -> Self {
        Error::SampleProgramCompilationFailed { name: name.into(), error: Box::new(error) }
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
                if arg.contains(' ') {
                    let arg = arg.replace('"', "\\\"");
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

#[derive(Debug, ThisError)]
pub(crate) enum FindBinaryInPathError {
    #[error("the PATH environment variable is not set")]
    NoEnvironmentVariable,
    #[error("binary {name} is not present in the system PATH")]
    MissingBinary { name: String },
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub(crate) enum LinkerArgsErrorKind {
    UnknownArgument,
    DisallowedPlugin,
    NoArgsFile,
    InvalidArgsFile,
    EmptyArgsFile,
    MissingArg,
}
