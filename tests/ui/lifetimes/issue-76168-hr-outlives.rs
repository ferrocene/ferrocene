//@ edition:2018
//@ check-pass

#![feature(unboxed_closures)]
use std::future::Future;

async fn wrapper<F>(f: F)
where for<'a> F: FnOnce<(&'a mut i32,)>,
    for<'a> <F as FnOnce<(&'a mut i32,)>>::Output: Future<Output=()> + 'a
{
    let mut i = 41;
    f(&mut i).await;
}

async fn add_one(i: &mut i32) {
    *i = *i + 1;
}

fn main() {}

// ferrocene-annotations: fls_qcb1n9c0e5hz
// Functions
//
// ferrocene-annotations: fls_jeoas4n6su4
// Trait and Lifetime Bounds
//
// ferrocene-annotations: fls_7nv8ualeaqe3
// Where Clauses
