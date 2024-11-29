<<<<<<< HEAD
#![cfg_attr(feature = "core", no_std)]
#![cfg_attr(not(feature = "core"), feature(no_core), no_core)]
=======
// tidy-alphabetical-start
#![allow(internal_features)]
#![feature(no_core)]
>>>>>>> pull-upstream-temp--do-not-use-for-real-code
#![feature(profiler_runtime)]
#![feature(staged_api)]
// tidy-alphabetical-end

// Other attributes:
#![no_core]
#![profiler_runtime]
#![unstable(
    feature = "profiler_runtime_lib",
    reason = "internal implementation detail of rustc right now",
    issue = "none"
)]
