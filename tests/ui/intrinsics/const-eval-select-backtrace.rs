#![feature(core_intrinsics)]
// See issue #100696.
//@ run-fail
//@ check-run-results
//@ exec-env:RUST_BACKTRACE=0

// Ferrocene addition: QEMU user space emulation outputs an extra message when an abort happens
//@ ignore-qemu

#[track_caller]
fn uhoh() {
    panic!("Aaah!")
}

const fn c() {}

fn main() {
    std::intrinsics::const_eval_select((), c, uhoh);
}
