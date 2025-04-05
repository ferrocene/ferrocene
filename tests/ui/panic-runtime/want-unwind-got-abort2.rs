//@ build-fail
//@ needs-unwind
//@ aux-build:panic-runtime-abort.rs
//@ aux-build:wants-panic-runtime-abort.rs
//@ aux-build:panic-runtime-lang-items.rs

#![no_std]
#![no_main]

extern crate wants_panic_runtime_abort;
extern crate panic_runtime_lang_items;

<<<<<<< HEAD
// ferrocene-annotations: fls_fh27ljezn3qz
// Attribute no_main
=======
//~? ERROR the linked panic runtime `panic_runtime_abort` is not compiled with this crate's panic strategy `unwind`
//~? ERROR the crate `wants_panic_runtime_abort` requires panic strategy `abort` which is incompatible with this crate's strategy of `unwind`
>>>>>>> pull-upstream-temp--do-not-use-for-real-code
