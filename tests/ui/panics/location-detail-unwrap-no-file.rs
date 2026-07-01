//@ run-fail
//@ check-run-results
//@ compile-flags: -Copt-level=0 -Zlocation-detail=line,column
//@ exec-env:RUST_BACKTRACE=0

// Ferrocene addition: QEMU user space emulation outputs an extra message when an abort happens
//@ ignore-aarch64-unknown-ferrocene.facade
//@ ignore-aarch64r82-unknown-ferrocene.facade
//@ ignore-aarch64v8r-unknown-ferrocene.facade
//@ ignore-armv7r-ferrocene.facade-eabihf
//@ ignore-thumbv7em-ferrocene.facade-eabi
//@ ignore-thumbv7em-ferrocene.facade-eabihf

fn main() {
    let opt: Option<u32> = None;
    opt.unwrap();
}
