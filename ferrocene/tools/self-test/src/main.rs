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

use crate::error::Error;
use crate::report::{Reporter, StderrReporter};
use std::ffi::OsString;
use std::path::PathBuf;

struct Environment {
    path: Option<OsString>,
}

impl Environment {
    fn gather() -> Self {
        Self { path: std::env::var_os("PATH") }
    }
}

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
