//@ dont-check-compiler-stderr

#![feature(lang_items)]
#![no_main]
#![no_std]

#[lang = "eh_personality"]
fn eh() {}

<<<<<<< HEAD
// ferrocene-annotations: fls_fh27ljezn3qz
// Attribute no_main
=======
//~? ERROR `#[panic_handler]` function required, but not found
>>>>>>> pull-upstream-temp--do-not-use-for-real-code
