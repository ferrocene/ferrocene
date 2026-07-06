//@ run-fail
//@ check-run-results
//@ exec-env:RUST_BACKTRACE=0

// Ferrocene addition: Remove extra output line added by qemu
//@ normalize-stderr: "qemu: uncaught target signal 6 \(Aborted\) - core dumped\n" -> ""

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
