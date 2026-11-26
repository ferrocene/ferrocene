// tidy-alphabetical-start
#![allow(internal_features)]
#![feature(profiler_runtime)]
#![feature(staged_api)]
// tidy-alphabetical-end

// Ferrocene modification: In order to generate coverage data for core, core needs to depend
// on this crate. However, upstream added a dependency the other way to fix a symbol-mangling
// issue:
//
// https://github.com/rust-lang/rust/pull/164693
// https://github.com/rust-lang/rust/pull/166216
//
// The discussion on the first of these PRs suggests that a compiler change is planned which will
// require the `Sized` trait to exist before any functions can be defined, even magic
// compiler-injected ones which don't appear in the source here.
//
// Trying to declare `Sized` here and re-export it in core turned into a horrible mess, so for now
// just revert the change. In future, we may need to either do that, or modify the symbol
// mangling to not require `Sized` to exist here.
#![feature(no_core)]
#![no_core]
//
// Other attributes:
#![profiler_runtime]
#![unstable(
    feature = "profiler_runtime_lib",
    reason = "internal implementation detail of rustc right now",
    issue = "none"
)]
