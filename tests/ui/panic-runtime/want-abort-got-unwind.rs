// ignore-tidy-linelength
//@ build-fail
//@ dont-check-compiler-stderr
//@ aux-build:panic-runtime-unwind.rs
//@ compile-flags:-C panic=abort

// NOTE: depending on the target's default panic strategy, there can be additional errors that
// complain about linking two panic runtimes (e.g. precompiled `panic_unwind` if target default
// panic strategy is unwind, in addition to `panic_runtime_unwind`). These additional errors will
// not be observed on targets whose default panic strategy is abort, where `panic_abort` is linked
// in instead.
//@ dont-require-annotations: ERROR

extern crate panic_runtime_unwind;

fn main() {}

//~? ERROR the linked panic runtime `panic_runtime_unwind` is not compiled with this crate's panic strategy `abort`
<<<<<<< HEAD
// FIXME: These errors are target-dependent, could be served by some "optional error" annotation
// instead of `dont-require-annotations`.
//FIXME~? ERROR cannot link together two panic runtimes: panic_unwind and panic_runtime_unwind
//FIXME~? ERROR the crate `panic_unwind` requires panic strategy `unwind` which is incompatible with this crate's strategy of `abort`

// ferrocene-annotations: um_rustc_C_panic
=======
>>>>>>> pull-upstream-temp--do-not-use-for-real-code
