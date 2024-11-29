//@ run-pass
#![allow(dead_code)]
// after fixing #9384 and implementing hygiene for match bindings,
// this now fails because the insertion of the 'y' into the match
// doesn't cause capture. Making this macro hygienic (as I've done)
// could very well make this test case completely pointless....


enum T {
    A(isize),
    B(usize)
}

macro_rules! test {
    ($id:ident, $e:expr) => (
        fn foo(t: T) -> isize {
            match t {
                T::A($id) => $e,
                T::B($id) => $e
            }
        }
    )
}

test!(y, 10 + (y as isize));

pub fn main() {
    foo(T::A(20));
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
