// Test that the macro backtrace facility works
//@ aux-build:ping.rs
//@ revisions: default -Zmacro-backtrace
//@[-Zmacro-backtrace] compile-flags: -Z macro-backtrace

#[macro_use] extern crate ping;

// a local macro
macro_rules! pong {
    () => { syntax error };
}
//~^^ ERROR expected one of
//~| ERROR expected one of
//~| ERROR expected one of

#[allow(non_camel_case_types)]
struct syntax;

fn main() {
    pong!();
    ping!();
    deep!();
}

// ferrocene-annotations: fls_e0a96eb6ux3y
// Attribute macro_export
//
// ferrocene-annotations: fls_qxjy0f758x5s
// Attribute macro_use
//
// ferrocene-annotations: fls_xa7lp0zg1ol2
// Declarative Macros
//
// ferrocene-annotations: fls_vnvt40pa48n8
// Macro Invocation
