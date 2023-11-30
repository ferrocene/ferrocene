// compile-flags:-C panic=abort
// error-pattern: language item required, but not found: `panic_info`

#![feature(lang_items)]
#![feature(no_core)]
#![no_core]
#![no_main]

#[panic_handler]
fn panic() -> ! {
    loop {}
}

#[lang = "sized"]
trait Sized {}

// ferrocene-annotations: fls_fh27ljezn3qz
// Attribute no_main
//
// ferrocene-annotations: um_rustc_C_panic
