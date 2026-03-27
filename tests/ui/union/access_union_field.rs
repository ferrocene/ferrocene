#![allow(unused_variables)]

union Foo {
    bar: i8,
    baz: u8,
}

fn main() {
    let foo = Foo { bar: 5 };
    let a = foo.bar; //~ ERROR access to union field is unsafe and requires unsafe function or block
    let b = foo.baz; //~ ERROR access to union field is unsafe and requires unsafe function or block
}

// ferrocene-annotations: fls_fmdn7n7s413d
// Union Types
//
// ferrocene-annotations: fls_8tsynkj2cufj
// Struct Expressions
//
// ferrocene-annotations: fls_18k3uajrgq5f
// Field Access Expressions
//
// ferrocene-annotations: fls_9kjpxri0axvg
// Weak Keywords
//
// ferrocene-annotations: fls_xcwfotmq2e5d
// Field Resolution
