//@ run-pass
#![allow(dead_code)]

enum T {
    A(isize),
    B(f64)
}

// after fixing #9384 and implementing hygiene for match bindings,
// this now fails because the insertion of the 'y' into the match
// doesn't cause capture. Making this macro hygienic (as I've done)
// could very well make this test case completely pointless....

macro_rules! test {
    ($id1:ident, $id2:ident, $e:expr) => (
        fn foo(a:T, b:T) -> T {
            match (a, b) {
                (T::A($id1), T::A($id2)) => T::A($e),
                (T::B($id1), T::B($id2)) => T::B($e),
                _ => panic!()
            }
        }
    )
}

test!(x,y,x + y);

pub fn main() {
    foo(T::A(1), T::A(2));
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
