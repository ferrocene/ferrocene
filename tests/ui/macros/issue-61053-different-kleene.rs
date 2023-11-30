#![deny(meta_variable_misuse)]

macro_rules! foo {
    () => {};
    ( $( $i:ident = $($j:ident),+ );* ) => { $( $( $i = $j; )* )* };
    //~^ ERROR meta-variable repeats with
    ( $( $($j:ident),+ );* ) => { $( $( $j; )+ )+ }; //~ERROR meta-variable repeats with
}

macro_rules! bar {
    () => {};
    (test) => {
        macro_rules! nested {
            () => {};
            ( $( $i:ident = $($j:ident),+ );* ) => { $( $( $i = $j; )* )* };
            //~^ ERROR meta-variable repeats with
            ( $( $($j:ident),+ );* ) => { $( $( $j; )+ )+ }; //~ERROR meta-variable repeats with
        }
    };
    ( $( $i:ident = $($j:ident),+ );* ) => {
        $(macro_rules! $i {
            () => { 0 $( + $j )* }; //~ ERROR meta-variable repeats with
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
