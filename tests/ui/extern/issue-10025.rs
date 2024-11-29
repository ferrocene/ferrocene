//@ run-pass
#![allow(dead_code)]

unsafe extern fn foo() {}
unsafe extern "C" fn bar() {}

fn main() {
    let _a: unsafe extern fn() = foo;
    let _a: unsafe extern "C" fn() = foo;
}

// ferrocene-annotations: fls_qcb1n9c0e5hz
// Functions
// ferrocene-annotations: fls_xztr1kebz8bo
// Function Pointer Types
// ferrocene-annotations: fls_usgd0xlijoxv
// ABI
