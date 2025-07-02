//! Ensure that macros produce an error if fragment specifiers are missing.

macro_rules! used_arm {
    ( $( any_token $field_rust_type )* ) => {}; //~ ERROR missing fragment
}

macro_rules! used_macro_unused_arm {
    () => {};
    ( $name ) => {}; //~ ERROR missing fragment
}

macro_rules! unused_macro {
    ( $name ) => {}; //~ ERROR missing fragment
}

fn main() {
    used_arm!();
    used_macro_unused_arm!();
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
