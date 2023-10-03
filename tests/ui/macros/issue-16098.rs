macro_rules! prob1 {
    (0) => {
        0
    };
    ($n:expr) => {
        if ($n % 3 == 0) || ($n % 5 == 0) {
            $n + prob1!($n - 1); //~ ERROR recursion limit reached while expanding `prob1!`
        } else {
            prob1!($n - 1);
        }
    };
}

fn main() {
    println!("Problem 1: {}", prob1!(1000));
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
