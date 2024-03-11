//@ run-pass
// #42164

#![feature(decl_macro)]
#![allow(dead_code)]

pub macro m($inner_str:expr) {
    #[doc = $inner_str]
    struct S;
}

macro_rules! define_f {
    ($name:expr) => {
        #[export_name = $name]
        fn f() {}
    }
}

fn main() {
    define_f!(concat!("exported_", "f"));
    m!(stringify!(foo));
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
// ferrocene-annotations: fls_olzilmy8n0nl
// Attribute export_name
