//@ run-pass
macro_rules! myfn {
    ( $f:ident, ( $( $x:ident ),* ), $body:block ) => (
        fn $f( $( $x : isize),* ) -> isize $body
    )
}

myfn!(add, (a,b), { return a+b; } );

pub fn main() {

    macro_rules! mylet {
        ($x:ident, $val:expr) => (
            let $x = $val;
        )
    }

    mylet!(y, 8*2);
    assert_eq!(y, 16);

    myfn!(mult, (a,b), { a*b } );

    assert_eq!(mult(2, add(4,4)), 16);

    macro_rules! actually_an_expr_macro {
        () => ( 16 )
    }

    assert_eq!({ actually_an_expr_macro!() }, 16);

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
