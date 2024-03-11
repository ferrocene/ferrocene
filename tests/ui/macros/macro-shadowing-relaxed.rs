//@ build-pass (FIXME(62277): could be check-pass?)
//@ aux-build:macro-in-other-crate.rs

#![feature(decl_macro)]

macro_rules! my_include {() => {
    // Outer
    macro m() {}
    #[macro_use(from_prelude)] extern crate macro_in_other_crate;

    fn inner() {
        // Inner
        macro m() {}
        macro_rules! from_prelude { () => {} }

        // OK, both `m` and `from_prelude` are macro-expanded,
        // but no more macro-expanded than their counterpart from outer scope.
        m!();
        from_prelude!();
    }
}}

my_include!();

fn main() {}

// ferrocene-annotations: fls_xa7lp0zg1ol2
// Declarative Macros
//
// ferrocene-annotations: fls_wjldgtio5o75
// Macro Expansion
//
// ferrocene-annotations: fls_vnvt40pa48n8
// Macro Invocation
//
// ferrocene-annotations: fls_ym00b6ewf4n3
// Macro Transcription
//
// ferrocene-annotations: fls_8nzypdu9j3ge
// Metavariables
//
// ferrocene-annotations: fls_ydmnb7qnmzzq
// Shadowing
