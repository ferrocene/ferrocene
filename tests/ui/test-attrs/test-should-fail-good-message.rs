//@ run-pass
//@ needs-unwind
//@ compile-flags: --test
#[test]
#[should_panic(expected = "foo")]
pub fn test_foo() {
    panic!("foo bar")
}

#[test]
#[should_panic(expected = "foo")]
pub fn test_foo_dynamic() {
    panic!("{} bar", "foo")
}

// ferrocene-annotations: fls_k02nt1m5fq1z
// Panic
//
// ferrocene-annotations: fls_aes2d94g12b9
// Attribute should_panic
//
// ferrocene-annotations: fls_zjoamsr3dbqk
// Diverging Expressions
//
// ferrocene-annotations: um_rustc_test
