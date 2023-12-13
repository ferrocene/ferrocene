#![feature(rustc_attrs)]

macro_rules! check {
    ($expr: expr) => (
        #[rustc_dummy = $expr]
        use main as _;
    );
}

check!("0"); // OK
check!(0); // OK
check!(0u8); //~ ERROR suffixed literals are not allowed in attributes
check!(-0); //~ ERROR attribute value must be a literal
check!(0 + 0); //~ ERROR attribute value must be a literal

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
