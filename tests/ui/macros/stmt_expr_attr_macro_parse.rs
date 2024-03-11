//@ run-pass
#![allow(unused_macro_rules)]

macro_rules! m {
    ($e:expr) => {
        "expr includes attr"
    };
    (#[$attr:meta] $e:expr) => {
        "expr excludes attr"
    }
}

macro_rules! n {
    (#[$attr:meta] $e:expr) => {
        "expr excludes attr"
    };
    ($e:expr) => {
        "expr includes attr"
    }
}

fn main() {
    assert_eq!(m!(#[attr] 1), "expr includes attr");
    assert_eq!(n!(#[attr] 1), "expr excludes attr");
}

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
// ferrocene-annotations: fls_n3ktmjqf87qb
// Rule Matching
//
// ferrocene-annotations: fls_qpx6lgapce57
// Token Matching
