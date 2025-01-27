//@ run-fail
//@ check-stdout
//@ compile-flags: --test
//@ needs-subprocess

#[test]
fn test_foo() {
    panic!()
}

// ferrocene-annotations: um_rustc_test
