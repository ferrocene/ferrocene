// Ferrocene addition: QEMU user space emulation outputs an extra message when an abort happens
//@ ignore-aarch64-unknown-ferrocene.facade
//@ ignore-aarch64r82-unknown-ferrocene.facade
//@ ignore-aarch64v8r-unknown-ferrocene.facade
//@ ignore-armv7r-ferrocene.facade-eabihf
//@ ignore-thumbv7em-ferrocene.facade-eabi
//@ ignore-thumbv7em-ferrocene.facade-eabihf

//@ run-fail
//@ check-run-results
//@ exec-env:RUST_BACKTRACE=0
//@ normalize-stderr: ".rs:\d+:\d+" -> ".rs:LL:CC"
//@ normalize-stderr: "/rustc(?:-dev)?/[a-z0-9.]+/" -> ""
//
// Regression test for issue #70963
// The reported panic location should not be `<::core::macros::panic macros>`.
fn main() {
    std::collections::VecDeque::<String>::with_capacity(!0);
}
