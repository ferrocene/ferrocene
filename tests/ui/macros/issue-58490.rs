// Regression test for #58490

macro_rules! a {
    ( @1 $i:item ) => {
        a! { @2 $i }
    };
    ( @2 $i:item ) => {
        $i
    };
}
mod b {
    a! {
        @1
        #[macro_export]
        macro_rules! b { () => () }
    }
    #[macro_export]
    macro_rules! b { () => () }
    //~^ ERROR: the name `b` is defined multiple times
}
mod c {
    #[allow(unused_imports)]
    use crate::b;
}

fn main() {}

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
