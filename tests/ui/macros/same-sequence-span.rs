//@ proc-macro: proc_macro_sequence.rs

// Regression test for issue #62831: Check that multiple sequences with the same span in the
// left-hand side of a macro definition behave as if they had unique spans, and in particular that
// they don't crash the compiler.

#![allow(unused_macros)]

extern crate proc_macro_sequence;

// When ignoring spans, this macro has the same macro definition as `generated_foo` in
// `proc_macro_sequence.rs`.
macro_rules! manual_foo {
    (1 $x:expr $($y:tt,)*   //~ERROR `$x:expr` may be followed by `$y:tt`
               $(= $z:tt)*  //~ERROR `$x:expr` may be followed by `=`
    ) => {};
}

proc_macro_sequence::make_foo!(); //~ERROR `$x:expr` may be followed by `$y:tt`
                                  //~^ERROR `$x:expr` may be followed by `=`

fn main() {}

// ferrocene-annotations: fls_xa7lp0zg1ol2
// Declarative Macros
//
// ferrocene-annotations: fls_8nzypdu9j3ge
// Metavariables
//
// ferrocene-annotations: fls_wjldgtio5o75
// Macro Expansion
//
// ferrocene-annotations: fls_vnvt40pa48n8
// Macro Invocation
//
// ferrocene-annotations: fls_4apk1exafxii
// Macro Matching
//
// ferrocene-annotations: fls_ym00b6ewf4n3
// Macro Transcription
//
// ferrocene-annotations: fls_k01lsksqtq1r
// Repetition
//
// ferrocene-annotations: fls_n3ktmjqf87qb
// Rule Matching
//
// ferrocene-annotations: fls_qpx6lgapce57
// Token Matching
//
// ferrocene-annotations: fls_wn1i6hzg2ff7
// Procedural Macros
