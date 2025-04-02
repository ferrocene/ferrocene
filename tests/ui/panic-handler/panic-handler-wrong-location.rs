//@ compile-flags:-C panic=abort

#![no_std]
#![no_main]

#[panic_handler] //~ ERROR `panic_impl` lang item must be applied to a function
#[no_mangle]
static X: u32 = 42;

//~? ERROR `#[panic_handler]` function required, but not found

// ferrocene-annotations: fls_fh27ljezn3qz
// Attribute no_main
//
// ferrocene-annotations: um_rustc_C_panic
