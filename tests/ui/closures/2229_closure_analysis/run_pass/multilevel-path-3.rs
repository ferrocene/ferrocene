//@ edition:2021
//@ run-pass

#![allow(unused)]

// Test that when `capture_disjoint_fields` is enabled we can read a path
// both inside and outside the closure at the same time.

struct Point {
    x: i32,
    y: i32,
}
struct Wrapper {
    p: Point,
}

fn main() {
    let mut w = Wrapper { p: Point { x: 10, y: 10 } };

    let c = || {
        println!("{}", w.p.x);
    };

    let px = &w.p.x;
    c();

    println!("{}", px);
}

// ferrocene-annotations: fls_18k3uajrgq5f
// Field Access Expressions
//
// ferrocene-annotations: fls_6ydylimiv553
// Place Expressions
//
// ferrocene-annotations: fls_xcwfotmq2e5d
// Field Resolution
