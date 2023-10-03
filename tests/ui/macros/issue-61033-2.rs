// Regression test for issue #61033.

macro_rules! test2 {
    (
        $(* $id1:ident)*
        $(+ $id2:ident)*
    ) => {
        $(
        //~^ ERROR meta-variable `id1` repeats 2 times
        //~| ERROR meta-variable `id1` repeats 2 times
            $id1 + $id2 // $id1 and $id2 may repeat different numbers of times
        )*
    }
}

fn main() {
    test2! {
        * a * b
        + a + b + c
    }
    test2! {
        * a * b
        + a + b + c + d
    }
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
//
// ferrocene-annotations: fls_9kjpxri0axvg
// Weak Keywords
