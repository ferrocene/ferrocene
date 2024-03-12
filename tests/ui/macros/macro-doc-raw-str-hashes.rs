//@ run-pass
// The number of `#`s used to wrap the documentation comment should differ regarding the content.
//
// Related issue: #27489

macro_rules! homura {
    ($x:expr, #[$y:meta]) => (assert_eq!($x, stringify!($y)))
}

fn main() {
    homura! {
        r#"doc = r" Madoka""#,
        /// Madoka
    };

    homura! {
        r##"doc = r#" One quote mark: ["]"#"##,
        /// One quote mark: ["]
    };

    homura! {
        r##"doc = r#" Two quote marks: [""]"#"##,
        /// Two quote marks: [""]
    };

    homura! {
        r#####"doc = r####" Raw string ending sequences: ["###]"####"#####,
        /// Raw string ending sequences: ["###]
    };
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
// ferrocene-annotations: fls_usr6iuwpwqqh
// Raw String Literals
//
// ferrocene-annotations: fls_k01lsksqtq1r
// Repetition
//
// ferrocene-annotations: fls_n3ktmjqf87qb
// Rule Matching
//
// ferrocene-annotations: fls_qpx6lgapce57
// Token Matching
