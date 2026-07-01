//@ run-fail
//@ check-run-results
//@ compile-flags: -Copt-level=0 -Zlocation-detail=line,column
//@ exec-env:RUST_BACKTRACE=0

// Ferrocene addition: QEMU user space emulation outputs an extra message when an abort happens
//@ ignore-qemu

fn main() {
    let opt: Option<u32> = None;
    opt.unwrap();
}
