//~ ERROR [E0463]
//@ check-fail
//@ compile-flags: --target=x86_64-unknown-none
//@ only-x86_64-unknown-linux-gnu
//@ needs-llvm-components:
fn main() {}

// ferrocene-annotations: um_rustc_target
