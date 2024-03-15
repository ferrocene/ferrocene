//@ run-fail
//@ compile-flags: --test
//@ check-stdout
//@ needs-unwind

#[test]
#[should_panic(expected = "foo")]
pub fn test_bar() {
    panic!("bar")
}

// ferrocene-annotations: um_rustc_test
