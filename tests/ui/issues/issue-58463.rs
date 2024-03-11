//@ run-pass
//@ compile-flags:-C debuginfo=2

fn foo() -> impl Copy {
    foo
}
fn main() {
    foo();
}

// ferrocene-annotations: um_rustc_C_debuginfo
