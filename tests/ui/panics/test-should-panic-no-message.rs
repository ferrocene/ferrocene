//@ run-fail
//@ compile-flags: --test
//@ check-stdout
//@ ignore-wasm32 no processes

#[test]
#[should_panic(expected = "foo")]
pub fn test_explicit() {
    panic!()
}

// ferrocene-annotations: um_rustc_test
