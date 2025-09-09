//@ compile-flags: --test

#[test]
#[should_panic = "foo"]
fn test1() {
    panic!();
}

#[test]
#[should_panic(expected)]
//~^ ERROR malformed `should_panic` attribute input
//~| NOTE expected this to be of the form `expected = "..."`
//~| NOTE for more information, visit
fn test2() {
    panic!();
}

#[test]
#[should_panic(expect)]
//~^ ERROR malformed `should_panic` attribute input
//~| NOTE the only valid argument here is "expected"
//~| NOTE for more information, visit
fn test3() {
    panic!();
}

#[test]
#[should_panic(expected(foo, bar))]
//~^ ERROR malformed `should_panic` attribute input
//~| NOTE expected this to be of the form `expected = "..."`
//~| NOTE for more information, visit
fn test4() {
    panic!();
}

#[test]
#[should_panic(expected = "foo", bar)]
//~^ ERROR malformed `should_panic` attribute input
//~| NOTE expected a single argument here
//~| NOTE for more information, visit
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
