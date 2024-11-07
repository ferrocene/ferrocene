// Tests that `?` is a Kleene op and not a macro separator in the 2015 edition.

//@ edition:2015

macro_rules! foo {
    ($(a)?) => {};
}

// The Kleene op `?` does not admit a separator before it.
macro_rules! baz {
    ($(a),?) => {}; //~ERROR the `?` macro repetition operator
}

macro_rules! barplus {
    ($(a)?+) => {}; // ok. matches "a+" and "+"
}

macro_rules! barstar {
    ($(a)?*) => {}; // ok. matches "a*" and "*"
}

pub fn main() {
    foo!();
    foo!(a);
    foo!(a?); //~ ERROR no rules expected `?`
    foo!(a?a); //~ ERROR no rules expected `?`
    foo!(a?a?a); //~ ERROR no rules expected `?`

    barplus!(); //~ERROR unexpected end of macro invocation
    barplus!(a); //~ERROR unexpected end of macro invocation
    barplus!(a?); //~ ERROR no rules expected `?`
    barplus!(a?a); //~ ERROR no rules expected `?`
    barplus!(a+);
    barplus!(+);

    barstar!(); //~ERROR unexpected end of macro invocation
    barstar!(a); //~ERROR unexpected end of macro invocation
    barstar!(a?); //~ ERROR no rules expected `?`
    barstar!(a?a); //~ ERROR no rules expected `?`
    barstar!(a*);
    barstar!(*);
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
