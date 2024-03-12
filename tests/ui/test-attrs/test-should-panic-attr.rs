//@ check-pass
//@ compile-flags: --test

#[test]
#[should_panic = "foo"]
fn test1() {
    panic!();
}

#[test]
#[should_panic(expected)]
//~^ WARN: argument must be of the form:
fn test2() {
    panic!();
}

#[test]
#[should_panic(expect)]
//~^ WARN: argument must be of the form:
fn test3() {
    panic!();
}

#[test]
#[should_panic(expected(foo, bar))]
//~^ WARN: argument must be of the form:
fn test4() {
    panic!();
}

#[test]
#[should_panic(expected = "foo", bar)]
//~^ WARN: argument must be of the form:
fn test5() {
    panic!();
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
