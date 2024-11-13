macro_rules! foo {
    ($a:ident) => ();
    ($a:ident, $b:ident) => ();
    ($a:ident, $b:ident, $c:ident) => ();
    ($a:ident, $b:ident, $c:ident, $d:ident) => ();
    ($a:ident, $b:ident, $c:ident, $d:ident, $e:ident) => ();
}

macro_rules! bar {
    ($lvl:expr, $($arg:tt)+) => {}
}

macro_rules! check {
    ($ty:ty, $expected:expr) => {};
    ($ty_of:expr, $expected:expr) => {};
}

fn main() {
    println!("{}" a);
    //~^ ERROR expected `,`, found `a`
    foo!(a b);
    //~^ ERROR no rules expected `b`
    foo!(a, b, c, d e);
    //~^ ERROR no rules expected `e`
    foo!(a, b, c d, e);
    //~^ ERROR no rules expected `d`
    foo!(a, b, c d e);
    //~^ ERROR no rules expected `d`
    bar!(Level::Error, );
    //~^ ERROR unexpected end of macro invocation
    check!(<str as Debug>::fmt, "fmt");
    check!(<str as Debug>::fmt, "fmt",);
    //~^ ERROR no rules expected `,`
}

// ferrocene-annotations: fls_xa7lp0zg1ol2
// Declarative Macros
//
// ferrocene-annotations: fls_8nzypdu9j3ge
// Metavariables
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
// ferrocene-annotations: fls_n3ktmjqf87qb
// Rule Matching
//
// ferrocene-annotations: fls_qpx6lgapce57
// Token Matching
