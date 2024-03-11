//@ run-pass

pub fn increment(x: usize) -> usize {
    x + 1
}

#[macro_export]
macro_rules! increment {
    ($x:expr) => ({
        use $crate::increment;
        increment($x)
    })
}

fn main() {
    assert_eq!(increment!(3), 4);
}

// ferrocene-annotations: fls_e0a96eb6ux3y
// Attribute macro_export
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
// ferrocene-annotations: fls_n3ktmjqf87qb
// Rule Matching
//
// ferrocene-annotations: fls_qpx6lgapce57
// Token Matching
