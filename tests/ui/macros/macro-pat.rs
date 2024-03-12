//@ run-pass

macro_rules! mypat {
    () => (
        Some('y')
    )
}

macro_rules! char_x {
    () => (
        'x'
    )
}

macro_rules! some {
    ($x:pat) => (
        Some($x)
    )
}

macro_rules! indirect {
    () => (
        some!(char_x!())
    )
}

macro_rules! ident_pat {
    ($x:ident) => (
        $x
    )
}

fn f(c: Option<char>) -> usize {
    match c {
        Some('x') => 1,
        mypat!() => 2,
        _ => 3,
    }
}

pub fn main() {
    assert_eq!(1, f(Some('x')));
    assert_eq!(2, f(Some('y')));
    assert_eq!(3, f(None));

    assert_eq!(1, match Some('x') {
        Some(char_x!()) => 1,
        _ => 2,
    });

    assert_eq!(1, match Some('x') {
        some!(char_x!()) => 1,
        _ => 2,
    });

    assert_eq!(1, match Some('x') {
        indirect!() => 1,
        _ => 2,
    });

    assert_eq!(3, {
        let ident_pat!(x) = 2;
        x+1
    });
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
