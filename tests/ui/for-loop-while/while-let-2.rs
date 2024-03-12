//@ run-pass

#[allow(dead_code)]
fn macros() {
    macro_rules! foo{
        ($p:pat, $e:expr, $b:block) => {{
            while let $p = $e $b
            //~^ WARN irrefutable `while let`
            //~| WARN irrefutable `while let`
        }}
    }
    macro_rules! bar{
        ($p:pat, $e:expr, $b:block) => {{
            foo!($p, $e, $b)
        }}
    }

    foo!(_a, 1, {
        println!("irrefutable pattern");
    });
    bar!(_a, 1, {
        println!("irrefutable pattern");
    });
}

pub fn main() {
    while let _a = 1 { //~ WARN irrefutable `while let`
        println!("irrefutable pattern");
        break;
    }
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
// ferrocene-annotations: fls_m6kd5i9dy8dx
// While Let Loops
//
// ferrocene-annotations: fls_9kjpxri0axvg
// Weak Keywords
