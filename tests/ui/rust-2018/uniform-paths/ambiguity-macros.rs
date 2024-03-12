//@ edition:2018

// This test is similar to `ambiguity.rs`, but with macros defining local items.

#![allow(non_camel_case_types)]

use std::io;
//~^ ERROR `std` is ambiguous

macro_rules! m {
    () => {
        mod std {
            pub struct io;
        }
    }
}
m!();

fn main() {}

// ferrocene-annotations: fls_xa7lp0zg1ol2
// Declarative Macros
//
// ferrocene-annotations: fls_vnvt40pa48n8
// Macro Invocation
//
// ferrocene-annotations: fls_wjldgtio5o75
// Macro Expansion
//
// ferrocene-annotations: fls_4apk1exafxii
// Macro Matching
//
// ferrocene-annotations: fls_n3ktmjqf87qb
// Rule Matching
//
// ferrocene-annotations: fls_qpx6lgapce57
// Token Matching
//
// ferrocene-annotations: fls_ym00b6ewf4n3
// Macro Transcription
//
// ferrocene-annotations: fls_xlfo7di0gsqz
// Hygiene
//
// ferrocene-annotations: fls_jdknpu3kf865
// Visibility
//
// ferrocene-annotations: fls_9gprp17h6t1q
// Use Imports
