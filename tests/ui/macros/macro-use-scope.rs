//@ aux-build:two_macros.rs

//@ build-pass (FIXME(62277): could be check-pass?)
#![allow(unused)]

fn f() {
    let _ = macro_one!();
}
#[macro_use(macro_one)] // Check that this macro is usable in the above function
extern crate two_macros;

fn g() {
    macro_two!();
}
macro_rules! m { () => {
    #[macro_use(macro_two)] // Check that this macro is usable in the above function
    extern crate two_macros as _two_macros;
} }
m!();


fn main() {}

// ferrocene-annotations: fls_qxjy0f758x5s
// Attribute macro_use
//
// ferrocene-annotations: fls_xa7lp0zg1ol2
// Declarative Macros
//
// ferrocene-annotations: fls_vnvt40pa48n8
// Macro Invocation
