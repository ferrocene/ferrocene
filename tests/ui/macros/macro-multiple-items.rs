//@ run-pass
macro_rules! make_foo {
    () => (
        struct Foo;

        impl Foo {
            fn bar(&self) {}
        }
    )
}

make_foo!();

pub fn main() {
    Foo.bar()
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
// ferrocene-annotations: fls_ym00b6ewf4n3
// Macro Transcription
