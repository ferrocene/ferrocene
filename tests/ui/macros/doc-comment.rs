//@ check-pass
// Tests that we properly handle a nested macro expansion
// involving a `#[doc]` attribute
#![deny(missing_docs)]
//! Crate docs

macro_rules! doc_comment {
    ($x:expr, $($tt:tt)*) => {
        #[doc = $x]
        $($tt)*
    }
}

macro_rules! make_comment {
    () => {
        doc_comment!("Function docs",
            pub fn bar() {}
        );
    }
}


make_comment!();

fn main() {}

// ferrocene-annotations: fls_63v1fqedzwfd
// Attribute doc
//
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
//
// ferrocene-annotations: fls_9kjpxri0axvg
// Weak Keywords
