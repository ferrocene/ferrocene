#![cfg_attr(feature = "core", no_std)]
#![cfg_attr(not(feature = "core"), feature(no_core), no_core)]

#![feature(profiler_runtime)]
#![profiler_runtime]
#![unstable(
    feature = "profiler_runtime_lib",
    reason = "internal implementation detail of rustc right now",
    issue = "none"
)]
#![allow(unused_features)]
#![allow(internal_features)]
#![feature(staged_api)]
