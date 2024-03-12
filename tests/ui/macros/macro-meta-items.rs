//@ run-pass
#![allow(dead_code)]
//@ compile-flags: --cfg foo

macro_rules! compiles_fine {
    ($at:meta) => {
        #[cfg($at)]
        static MISTYPED: () = "foo";
    }
}
macro_rules! emit {
    ($at:meta) => {
        #[cfg($at)]
        static MISTYPED: &'static str = "foo";
    }
}

// item
compiles_fine!(bar);
emit!(foo);

fn foo() {
    println!("{}", MISTYPED);
}

pub fn main() {
    // statement
    compiles_fine!(baz);
    emit!(baz);
    println!("{}", MISTYPED);
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
//
// ferrocene-annotations: fls_9kjpxri0axvg
// Weak Keywords
//
// ferrocene-annotations: um_rustc_cfg
