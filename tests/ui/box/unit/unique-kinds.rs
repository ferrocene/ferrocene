//@ run-pass

use std::fmt::Debug;

fn sendable() {

    fn f<T:Send + PartialEq + Debug>(i: T, j: T) {
        assert_eq!(i, j);
    }

    fn g<T:Send + PartialEq>(i: T, j: T) {
        assert!(i != j);
    }

    let i: Box<_> = Box::new(100);
    let j: Box<_> = Box::new(100);
    f(i, j);
    let i: Box<_> = Box::new(100);
    let j: Box<_> = Box::new(101);
    g(i, j);
}

fn copyable() {

    fn f<T:PartialEq + Debug>(i: T, j: T) {
        assert_eq!(i, j);
    }

    fn g<T:PartialEq>(i: T, j: T) {
        assert!(i != j);
    }

    let i: Box<_> = Box::new(100);
    let j: Box<_> = Box::new(100);
    f(i, j);
    let i: Box<_> = Box::new(100);
    let j: Box<_> = Box::new(101);
    g(i, j);
}

fn noncopyable() {

    fn f<T:PartialEq + Debug>(i: T, j: T) {
        assert_eq!(i, j);
    }

    fn g<T:PartialEq>(i: T, j: T) {
        assert!(i != j);
    }

    let i: Box<_> = Box::new(100);
    let j: Box<_> = Box::new(100);
    f(i, j);
    let i: Box<_> = Box::new(100);
    let j: Box<_> = Box::new(101);
    g(i, j);
}

pub fn main() {
    sendable();
    copyable();
    noncopyable();
}

// ferrocene-annotations: fls_qcb1n9c0e5hz
// Functions
//
// ferrocene-annotations: fls_vhpwge5123cm
// Generic Parameters
//
// ferrocene-annotations: fls_jeoas4n6su4
// Trait and Lifetime Bounds
//
// ferrocene-annotations: fls_eiw4by8z75di
// Send and Sync
//
// ferrocene-annotations: fls_xa4nbfas01cj
// Call Expressions
//
// ferrocene-annotations: fls_77scxuomlbgs
// Passing Conventions
