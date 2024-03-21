// SPDX-License-Identifier: MIT OR Apache-2.0
// SPDX-FileCopyrightText: The Ferrocene Developers

//! Environment variables which act as input parameters.
//!
//! There are compile-time and run-time environment variables. Compile-time
//! variables are constants. Run-time variables are fields of [Env].

use std::ffi::OsString;

// Compile-time environment variables
//
/// The Rust release ferrocene-self-test is being compiled for
pub const CFG_RELEASE: &str = env!("CFG_RELEASE");
/// The target-triple ferroce-self-test is being compiled for
pub const SELFTEST_TARGET: &str = env!("SELFTEST_TARGET");
pub const SELFTEST_RUST_HASH: Option<&str> = option_env!("SELFTEST_RUST_HASH");
pub const SELFTEST_CARGO_HASH: Option<&str> = option_env!("SELFTEST_CARGO_HASH");

/// Run-time environment variables
pub(crate) struct Env {
    /// `PATH` environment variable.
    ///
    /// Specifies a set of directories where executable programs are located.
    ///
    /// Usually this variable is present in most operating systems and does not
    /// need to be set explicitly.
    pub path: Option<OsString>,
}

impl Env {
    pub fn gather() -> Self {
        Self { path: std::env::var_os("PATH") }
    }
}
