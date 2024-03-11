//@ run-pass
#![allow(stable_features)]
#![allow(unused_labels)]
#![allow(unreachable_code)]

macro_rules! x {
    ($a:lifetime) => {
        $a: loop {
            break $a;
            panic!("failed");
        }
    }
}
macro_rules! br {
    ($a:lifetime) => {
        break $a;
    }
}
macro_rules! br2 {
    ($b:lifetime) => {
        'b: loop {
            break $b; // this $b should refer to the outer loop.
        }
    }
}
fn main() {
    x!('a);
    'c: loop {
        br!('c);
        panic!("failed");
    }
    'b: loop {
        br2!('b);
        panic!("failed");
    }
}

// ferrocene-annotations: fls_xa7lp0zg1ol2
// Declarative Macros
//
// ferrocene-annotations: fls_sf4qnd43z2wc
// Infinite Loops
//
// ferrocene-annotations: fls_yqcygq3y6m5j
// Lifetime
//
// ferrocene-annotations: fls_uusi0zej55is
// Loop Labels
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
