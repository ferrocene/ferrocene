#![feature(core_intrinsics)]
// See issue #100696.
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

#[track_caller]
fn uhoh() {
    panic!("Aaah!")
}

const fn c() {}

fn main() {
    std::intrinsics::const_eval_select((), c, uhoh);
}
