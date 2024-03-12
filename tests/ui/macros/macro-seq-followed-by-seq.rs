//@ run-pass
// Test of allowing two sequences repetitions in a row,
// functionality added as byproduct of RFC amendment #1384
//   https://github.com/rust-lang/rfcs/pull/1384

// Old version of Rust would reject this macro definition, even though
// there are no local ambiguities (the initial `banana` and `orange`
// tokens are enough for the expander to distinguish which case is
// intended).
macro_rules! foo {
    ( $(banana $a:ident)* $(orange $b:tt)* ) => { };
}

fn main() {
    foo!( banana id1 banana id2
          orange hi  orange (hello world) );
}

// ferrocene-annotations: fls_xa7lp0zg1ol2
// Declarative Macros
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
// ferrocene-annotations: fls_8nzypdu9j3ge
// Metavariables
//
// ferrocene-annotations: fls_k01lsksqtq1r
// Repetition
//
// ferrocene-annotations: fls_n3ktmjqf87qb
// Rule Matching
//
// ferrocene-annotations: fls_qpx6lgapce57
// Token Matching
