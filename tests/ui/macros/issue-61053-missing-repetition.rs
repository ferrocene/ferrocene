#![deny(meta_variable_misuse)]

macro_rules! foo {
    () => {};
    ($( $i:ident = $($j:ident),+ );*) => { $( $i = $j; )* };
    //~^ ERROR variable 'j' is still repeating
}

macro_rules! bar {
    () => {};
    (test) => {
        macro_rules! nested {
            () => {};
            ($( $i:ident = $($j:ident),+ );*) => { $( $i = $j; )* };
            //~^ ERROR variable 'j' is still repeating
        }
    };
    ( $( $i:ident = $($j:ident),+ );* ) => {
        $(macro_rules! $i {
            () => { $j }; //~ ERROR variable 'j' is still repeating
        })*
    };
}

fn main() {
    foo!();
    bar!();
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
