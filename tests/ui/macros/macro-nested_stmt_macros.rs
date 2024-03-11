//@ run-pass
macro_rules! foo {
    () => {
        struct Bar;
        struct Baz;
    }
}

macro_rules! grault {
    () => {
        foo!();
        struct Xyzzy;
    }
}

fn static_assert_exists<T>() { }

fn main() {
    grault!();
    static_assert_exists::<Bar>();
    static_assert_exists::<Baz>();
    static_assert_exists::<Xyzzy>();
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
// ferrocene-annotations: fls_n3ktmjqf87qb
// Rule Matching
//
// ferrocene-annotations: fls_qpx6lgapce57
// Token Matching
