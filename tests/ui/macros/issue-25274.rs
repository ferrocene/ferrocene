//@ run-pass

macro_rules! test {
    (
        fn fun() -> Option<Box<$t:ty>>;
    ) => {
        fn fun(x: $t) -> Option<Box<$t>>
        { Some(Box::new(x)) }
    }
}

test! {
    fn fun() -> Option<Box<i32>>;
}

fn main() {
    println!("{}", fun(0).unwrap());
}

// ferrocene-annotations: fls_xa7lp0zg1ol2
// Declarative Macros
//
// ferrocene-annotations: fls_wjldgtio5o75
// Macro Expansion
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
