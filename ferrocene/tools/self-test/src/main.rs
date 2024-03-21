// SPDX-License-Identifier: MIT OR Apache-2.0
// SPDX-FileCopyrightText: The Ferrocene Developers

mod binaries;
mod compile;
mod env;
mod error;
mod linkers;
mod report;
mod targets;
mod utils;

#[cfg(test)]
mod test_utils;

use crate::env::Env;
use crate::error::Error;
use crate::report::{Reporter, StderrReporter};
use crate::utils::get_sysroot;

fn main_inner(reporter: &dyn Reporter) -> Result<(), Error> {
    let env = Env::gather();

    let sysroot = get_sysroot().ok_or(Error::NoSysroot)?;
    reporter.info(&format!("using sysroot {}", sysroot.display()));

    binaries::check(reporter, &sysroot)?;
    let mut targets = targets::check(reporter, &sysroot)?;
    linkers::check_and_add_rustflags(reporter, &env, &sysroot, &mut targets)?;
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
