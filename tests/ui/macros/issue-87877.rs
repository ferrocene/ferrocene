//@ check-pass

macro_rules! two_items {
    () => {
        extern "C" {}
        extern "C" {}
    };
}

macro_rules! single_expr_funneler {
    ($expr:expr) => {
        $expr; // note the semicolon, it changes the statement kind during parsing
    };
}

macro_rules! single_item_funneler {
    ($item:item) => {
        $item
    };
}

fn main() {
    single_expr_funneler! { two_items! {} }
    single_item_funneler! { two_items! {} }
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
