//@ dont-check-compiler-stderr
//@ error-pattern: `#[panic_handler]` function required, but not found

#![feature(lang_items)]
#![no_main]
#![no_std]

#[lang = "eh_personality"]
fn eh() {}

// ferrocene-annotations: fls_fh27ljezn3qz
// Attribute no_main
