//@ run-fail
//@ check-run-results
//@ exec-env:RUST_BACKTRACE=0

// Ferrocene addition: QEMU user space emulation outputs an extra message when an abort happens
//@ ignore-aarch64-unknown-ferrocene.facade
//@ ignore-aarch64r82-unknown-ferrocene.facade
//@ ignore-aarch64v8r-unknown-ferrocene.facade
//@ ignore-armv7r-ferrocene.facade-eabihf
//@ ignore-thumbv7em-ferrocene.facade-eabi
//@ ignore-thumbv7em-ferrocene.facade-eabihf

// Test that we format the panic message only once.
// Regression test for https://github.com/rust-lang/rust/issues/110717

use std::fmt;

struct PrintOnFmt;

impl fmt::Display for PrintOnFmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        eprintln!("fmt");
        f.write_str("PrintOnFmt")
    }
}

fn main() {
    panic!("{}", PrintOnFmt)
}
