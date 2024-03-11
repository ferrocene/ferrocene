//@ compile-flags:-C panic=unwind
//@ no-prefer-dynamic

#![crate_type = "rlib"]
#![no_std]

extern "C-unwind" fn foo() {}

fn bar() {
    let ptr: extern "C-unwind" fn() = foo;
    ptr();
}

// ferrocene-annotations: um_rustc_C_panic
