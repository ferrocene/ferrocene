//@ build-fail
//@ aux-build:needs-unwind.rs
//@ compile-flags:-C panic=abort
//@ no-prefer-dynamic

extern crate needs_unwind;

fn main() {}

<<<<<<< HEAD
// ferrocene-annotations: um_rustc_C_panic
=======
//~? ERROR the crate `needs_unwind` requires panic strategy `unwind` which is incompatible with this crate's strategy of `abort`
>>>>>>> pull-upstream-temp--do-not-use-for-real-code
