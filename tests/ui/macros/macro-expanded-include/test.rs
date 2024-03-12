//@ needs-asm-support
//@ build-pass (FIXME(62277): could be check-pass?)
#![allow(unused)]

#[macro_use]
mod foo;

m!();
fn f() {
    n!();
}

fn main() {}

// ferrocene-annotations: fls_qxjy0f758x5s
// Attribute macro_use
//
// ferrocene-annotations: fls_xa7lp0zg1ol2
// Declarative Macros
//
// ferrocene-annotations: fls_vnvt40pa48n8
// Macro Invocation
