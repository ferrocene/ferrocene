// Regression test for #84632: Recursion limit is ignored
// for builtin macros that eagerly expands.

#![recursion_limit = "15"]
macro_rules! a {
    () => ("");
    (A) => (concat!("", a!()));
    (A, $($A:ident),*) => (concat!("", a!($($A),*)))
    //~^ ERROR recursion limit reached
    //~| HELP consider increasing the recursion limit
}

fn main() {
    a!(A, A, A, A, A);
    a!(A, A, A, A, A, A, A, A, A, A, A);
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
