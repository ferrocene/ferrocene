// SPDX-License-Identifier: MIT OR Apache-2.0
// SPDX-FileCopyrightText: The Ferrocene Developers

mod binaries;
mod compile;
mod error;
mod linkers;
mod report;
mod targets;
mod utils;

#[cfg(test)]
mod test_utils;

use std::ffi::OsString;
use std::path::PathBuf;

use crate::error::Error;
use crate::report::{Reporter, StderrReporter};

// Compile-time environment variables
/// The Rust release ferrocene-self-test is being compiled for
const CFG_RELEASE: &str = env!("CFG_RELEASE");
/// The target-triple ferroce-self-test is being compiled for
const SELFTEST_TARGET: &str = env!("SELFTEST_TARGET");
const SELFTEST_RUST_HASH: Option<&str> = option_env!("SELFTEST_RUST_HASH");
const SELFTEST_CARGO_HASH: Option<&str> = option_env!("SELFTEST_CARGO_HASH");

/// Run-time environment variables set by the caller of the binary.
struct Environment {
    path: Option<OsString>,
}

impl Environment {
    fn gather() -> Self {
        Self { path: std::env::var_os("PATH") }
    }
}

/// The user manual states to extract all archives to the same directory.
/// Therefore the sysroot is the grandparent of `ferrocene-self-test`
/// (`PATH_TO_INSTALLATION_DIRECTORY/bin/ferrocene-self-test`).
fn get_sysroot() -> Option<PathBuf> {
    let current_exe = std::env::current_exe().ok()?;
    Some(current_exe.parent()?.parent()?.to_path_buf())
}

fn main_inner(reporter: &dyn Reporter) -> Result<(), Error> {
    let environment = Environment::gather();

    let sysroot = get_sysroot().ok_or(Error::NoSysroot)?;
    reporter.info(&format!("using sysroot {}", sysroot.display()));

    binaries::check(reporter, &sysroot)?;
    let mut targets = targets::check(reporter, &sysroot)?;
    linkers::check_and_add_rustflags(reporter, &environment, &sysroot, &mut targets)?;
    compile::check(reporter, &sysroot, &targets)?;
    linkers::report_linker_flags(reporter, &targets);

    reporter.success("Ferrocene self-check completed!");
    Ok(())
}

fn main() {
    let reporter = if atty::is(atty::Stream::Stderr) {
        StderrReporter::color()
    } else {
        StderrReporter::plain()
    };

    match main_inner(&reporter) {
        Ok(()) => {}
        Err(err) => {
            reporter.error(&err);
            std::process::exit(err.code() as i32);
        }
    }
}
