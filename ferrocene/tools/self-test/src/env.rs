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
pub(crate) struct Env {
    /// `PATH` environment variable.
    ///
    /// Specifies a set of directories where executable programs are located.
    ///
    /// Usually this variable is present in most operating systems and does not
    /// need to be set explicitly.
    pub(crate) path: OsString,
}

impl Env {
    pub(crate) fn gather() -> Result<Self, Error> {
        Ok(Self {
            path: std::env::var_os("PATH").ok_or_else(|| Error::CCompilerNotFound {
                error: crate::error::FindBinaryInPathError::NoEnvironmentVariable,
            })?,
        })
    }
}
