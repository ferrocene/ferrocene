//@ edition:2021
//@check-pass

fn test1() {
    let foo : [Vec<u8>; 3] = ["String".into(), "String".into(), "String".into()];
    let c = || {
        match foo { _ => () };
    };
    drop(foo);
    c();
}

fn test2() {
    let foo : Option<[Vec<u8>; 3]> = Some(["String".into(), "String".into(), "String".into()]);
    let c = || {
        match foo {
            Some(_) => 1,
            _ => 2
        };
    };
    c();
}

fn main() {
    test1();
    test2();
}

// ferrocene-annotations: fls_e5td0fa92fay
// Match Expressions
//
// ferrocene-annotations: fls_qfsfnql1t7m
// Wildcard Patterns
//
// ferrocene-annotations: fls_yc4xm4hrfyw7
// Wildcard Pattern Matching
