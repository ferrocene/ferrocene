//@ run-pass
#![allow(unused_macro_rules)]

/// A compile-time map from identifiers to arbitrary (heterogeneous) expressions
macro_rules! ident_map {
    ( $name:ident = { $($key:ident => $e:expr,)* } ) => {
        macro_rules! $name {
            $(
                ( $key ) => { $e };
            )*
            // Empty invocation expands to nothing. Needed when the map is empty.
            () => {};
        }
    };
}

ident_map!(my_map = {
    main => 0,
});

fn main() {
    my_map!(main);
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
