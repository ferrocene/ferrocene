//@ run-fail
//@ check-run-results
//@ compile-flags: -Zlocation-detail=file,column
//@ exec-env:RUST_BACKTRACE=0

// Ferrocene addition: QEMU user space emulation outputs an extra message when an abort happens
//@ ignore-qemu

fn main() {
    panic!("line-redacted");
}
