//@ run-pass
#![allow(unused_variables)]
macro_rules! foo {
    ($l:lifetime, $l2:lifetime) => {
        fn f<$l: $l2, $l2>(arg: &$l str, arg2: &$l2 str) -> &$l str {
            arg
        }
    }
}

pub fn main() {
    foo!('a, 'b);
    let x: &'static str = f("hi", "there");
    assert_eq!("hi", x);
}

// ferrocene-annotations: fls_xa7lp0zg1ol2
// Declarative Macros
//
// ferrocene-annotations: fls_yqcygq3y6m5j
// Lifetime
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
