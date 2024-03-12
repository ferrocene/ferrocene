//@ run-pass

// Test that a macro can correctly expand the alias
// in an `extern crate self as ALIAS` item.

fn the_answer() -> usize { 42 }

macro_rules! alias_self {
    ($alias:ident) => { extern crate self as $alias; }
}

alias_self!(the_alias);

fn main() {
    assert_eq!(the_alias::the_answer(), 42);
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
// ferrocene-annotations: fls_n3ktmjqf87qb
// Rule Matching
//
// ferrocene-annotations: fls_qpx6lgapce57
// Token Matching
