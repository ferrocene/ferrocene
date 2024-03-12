//@ run-pass

#![allow(unused_mut)]

// Check that when `?` is followed by what looks like a Kleene operator (?, +, and *)
// then that `?` is not interpreted as a separator. In other words, `$(pat)?+` matches `pat +`
// or `+` but does not match `pat` or `pat ? pat`.

//@ edition:2015

macro_rules! foo {
    // Check for `?`.
    ($($a:ident)? ? $num:expr) => {
        foo!($($a)? ; $num);
    };
    // Check for `+`.
    ($($a:ident)? + $num:expr) => {
        foo!($($a)? ; $num);
    };
    // Check for `*`.
    ($($a:ident)? * $num:expr) => {
        foo!($($a)? ; $num);
    };
    // Check for `;`, not a kleene operator.
    ($($a:ident)? ; $num:expr) => {
        let mut x = 0;

        $(
            x += $a;
        )?

        assert_eq!(x, $num);
    };
}

pub fn main() {
    let a = 1;

    // Accept 0 repetitions.
    foo!( ; 0);
    foo!( + 0);
    foo!( * 0);
    foo!( ? 0);

    // Accept 1 repetition.
    foo!(a ; 1);
    foo!(a + 1);
    foo!(a * 1);
    foo!(a ? 1);
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
