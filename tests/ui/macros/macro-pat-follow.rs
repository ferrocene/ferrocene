//@ run-pass
macro_rules! pat_in {
    ($p:pat in $e:expr) => {{
        let mut iter = $e.into_iter();
        while let $p = iter.next() {}
    }};
}

macro_rules! pat_if {
    ($p:pat if $e:expr) => {{
        match Some(1u8) {
            $p if $e => {}
            _ => {}
        }
    }};
}

fn main() {
    pat_in!(Some(_) in 0..10);
    pat_if!(Some(x) if x > 0);
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
