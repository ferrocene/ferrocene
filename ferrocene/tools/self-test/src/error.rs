// SPDX-License-Identifier: MIT OR Apache-2.0
// SPDX-FileCopyrightText: The Ferrocene Developers

use std::ffi::OsString;
use std::fmt::Display;
use std::path::PathBuf;
use std::process::Output;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub(crate) enum LinkerArgsErrorKind {
    UnknownArgument,
    DisallowedPlugin,
    NoArgsFile,
    InvalidArgsFile,
    EmptyArgsFile,
    MissingArg,
}

#[derive(Debug)]
pub(crate) enum Error {
    NoSysroot,
    MissingBinary { directory: PathBuf, name: String },
    WrongBinaryPermissions { path: PathBuf },
    MetadataFetchFailed { path: PathBuf, error: std::io::Error },
    VersionFetchFailed { binary: String, error: Box<CommandError> },
    VersionParseFailed { binary: String },
    BinaryVersionMismatch { binary: String, field: String, expected: String, found: String },
    TargetLibraryMissing { target: String, library: String },
    DuplicateTargetLibrary { target: String, library: String },
    TargetLibraryDiscoveryFailed { path: PathBuf, error: std::io::Error },
    CCompilerNotFound { name: String, error: FindBinaryInPathError },
    BundledLinkerMissing,
    NonUtf8Path { path: PathBuf },
    TemporaryCompilationDirectoryCreationFailed { error: std::io::Error },
    WritingSampleProgramFailed { name: String, dest: PathBuf, error: std::io::Error },
    SampleProgramCompilationFailed { name: String, error: Box<CommandError> },
    CompilationArtifactsListingFailed { path: PathBuf, error: std::io::Error },
    MissingCompilationArtifact { name: String, after_compiling: String },
    UnexpectedCompilationArtifact { name: String, after_compiling: String },
    SuitableCCompilerNotFound { target: String },
    WrongLinkerArgs { target: String, kind: LinkerArgsErrorKind },
    RunningSampleProgramFailed { name: String, error: std::io::Error },
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
            Error::CCompilerNotFound { error, .. } => Some(error),
            Error::BundledLinkerMissing => None,
            Error::NonUtf8Path { .. } => None,
            Error::TemporaryCompilationDirectoryCreationFailed { error } => Some(error),
            Error::WritingSampleProgramFailed { error, .. } => Some(error),
            Error::SampleProgramCompilationFailed { error, .. } => Some(error),
            Error::CompilationArtifactsListingFailed { error, .. } => Some(error),
            Error::MissingCompilationArtifact { .. } => None,
            Error::UnexpectedCompilationArtifact { .. } => None,
            Error::SuitableCCompilerNotFound { .. } => None,
            Error::WrongLinkerArgs { .. } => None,
            Error::RunningSampleProgramFailed { error, .. } => Some(error),
            Error::SampleProgramOutputWrong { .. } => None,
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
            Error::CCompilerNotFound { name, .. } => {
                write!(f, "C compiler {name} not found on the system",)
            }
            Error::BundledLinkerMissing => {
                write!(f, "the bundled linker is missing")
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
            Error::SuitableCCompilerNotFound { target } => {
                write!(f, "unable to find LLD-compatible C compiler for `{target}`")
            }
            Error::WrongLinkerArgs { target, kind } => {
                write!(f, "Unable to analyse linker arguments for `{target}` (kind={kind:?})")
            }
            Error::RunningSampleProgramFailed { name, .. } => {
                write!(f, "unable to execute sample program {name}")
            }
            Error::SampleProgramOutputWrong { name, expected, found } => {
                write!(
                    f,
                    "sample program {name} should have produced {expected:?}, actually produced {found:?}"
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
