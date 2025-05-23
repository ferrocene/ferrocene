//@ run-pass
#![allow(unused_variables)]
// Test implicit coercions from a fn item type to a fn pointer type.


fn foo(x: isize) -> isize { x * 2 }
fn bar(x: isize) -> isize { x * 4 }
type IntMap = fn(isize) -> isize;

fn eq<T>(x: T, y: T) { }

fn main() {
    let f: IntMap = foo;

    eq::<IntMap>(foo, bar);
}

// ferrocene-annotations: fls_airvr79xkcag
// Function Item Type
//
// ferrocene-annotations: fls_xztr1kebz8bo
// Function Pointer Type
//
// ferrocene-annotations: fls_dw33yt5g6m0k
// Type Coercion
