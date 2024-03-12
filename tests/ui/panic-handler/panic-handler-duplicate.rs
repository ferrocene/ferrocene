//@ compile-flags:-C panic=abort

#![feature(lang_items)]
#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    loop {}
}

#[lang = "panic_impl"]
fn panic2(info: &PanicInfo) -> ! { //~ ERROR found duplicate lang item `panic_impl`
    loop {}
}

// ferrocene-annotations: fls_fh27ljezn3qz
// Attribute no_main
//
// ferrocene-annotations: um_rustc_C_panic
