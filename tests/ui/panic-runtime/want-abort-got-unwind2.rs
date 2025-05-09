// ignore-tidy-linelength
//@ build-fail
//@ dont-check-compiler-stderr
//@ aux-build:panic-runtime-unwind.rs
//@ aux-build:wants-panic-runtime-unwind.rs
//@ compile-flags:-C panic=abort

// Like `want-abort-got-unwind.rs`, this version checks that if the root binary wants abort panic
// runtime, that the compiler rejects a setup where a dependency crate in the dependency DAG
// transitively provides an unwind panic runtime (which also is built with `-Cpanic=unwind`, making
// that potentially-unwinding).

// NOTE: similar to `want-abort-got-unwind.rs`, there can be additional errors if the target default
// panic strategy is unwind, because then the precompiled `panic_unwind` would also be linked in,
// duplicating `panic_runtime_unwind` (transitively). But those additional errors are not important
// to test intention.
//@ dont-require-annotations: ERROR

extern crate wants_panic_runtime_unwind;

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
