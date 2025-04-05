//@ build-fail
//@ needs-unwind
//@ aux-build:panic-runtime-abort.rs
//@ aux-build:panic-runtime-lang-items.rs

#![no_std]
#![no_main]

extern crate panic_runtime_abort;
extern crate panic_runtime_lang_items;

<<<<<<< HEAD
// ferrocene-annotations: fls_fh27ljezn3qz
// Attribute no_main
=======
//~? ERROR the linked panic runtime `panic_runtime_abort` is not compiled with this crate's panic strategy `unwind`
>>>>>>> pull-upstream-temp--do-not-use-for-real-code
