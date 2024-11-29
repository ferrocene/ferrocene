//@ run-pass
//@ aux-build:extern_calling_convention.rs


extern crate extern_calling_convention;

use extern_calling_convention::foo;

pub fn main() {
    foo(1, 2, 3, 4);
}

// ferrocene-annotations: fls_qcb1n9c0e5hz
// Functions
// ferrocene-annotations: fls_xa4nbfas01cj
// Call Expressions
