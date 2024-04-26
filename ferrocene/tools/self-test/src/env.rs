// SPDX-License-Identifier: MIT OR Apache-2.0
// SPDX-FileCopyrightText: The Ferrocene Developers

//! Environment variables which act as input parameters.
//!
//! There are compile-time and run-time environment variables. Compile-time
//! variables are constants. Run-time variables are fields of [Env].

use std::ffi::OsString;

use crate::error::Error;

// Compile-time environment variables
//
/// The Rust release ferrocene-self-test is being compiled for
pub(crate) const CFG_RELEASE: &str = env!("CFG_RELEASE");
/// The target-triple ferroce-self-test is being compiled for
pub(crate) const SELFTEST_TARGET: &str = env!("SELFTEST_TARGET");
pub(crate) const SELFTEST_RUST_HASH: Option<&str> = option_env!("SELFTEST_RUST_HASH");
pub(crate) const SELFTEST_CARGO_HASH: Option<&str> = option_env!("SELFTEST_CARGO_HASH");

/// Run-time environment variables
#[derive(Debug, Default)]
pub(crate) struct Env {
    /// `PATH` environment variable.
    ///
    /// Specifies a set of directories where executable programs are located.
    ///
    /// Usually this variable is present in most operating systems and does not
    /// need to be set explicitly.
    pub(crate) path: OsString,
    pub(crate) print_detailed_args: bool,
    pub(crate) print_detailed_errors: bool,
}

impl Env {
    pub(crate) fn gather() -> Result<Self, Error> {
        Self::new(
            std::env::var_os("PATH"),
            std::env::var("FST_PRINT_DETAILED_ARGS").is_ok(),
            std::env::var("FST_PRINT_DETAILED_ERRORS").is_ok(),
        )
    }

    fn new(
        path: Option<OsString>,
        print_detailed_args: bool,
        print_detailed_errors: bool,
    ) -> Result<Self, Error> {
        Ok(Self {
            path: path.ok_or_else(|| Error::CCompilerNotFound {
                error: crate::error::FindBinaryInPathError::NoEnvironmentVariable,
            })?,
            print_detailed_args,
            print_detailed_errors,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_missing_path() {
        let err = Env::new(None, false, false).unwrap_err();
        assert!(matches!(
            err,
            Error::CCompilerNotFound {
                error: crate::error::FindBinaryInPathError::NoEnvironmentVariable,
            }
        ));
    }
}
