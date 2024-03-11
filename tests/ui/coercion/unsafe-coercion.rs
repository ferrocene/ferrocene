//@ run-pass
// Check that safe fns are not a subtype of unsafe fns.


fn foo(x: i32) -> i32 {
    x * 22
}

fn bar(x: fn(i32) -> i32) -> unsafe fn(i32) -> i32 {
    x // OK, coercion!
}

fn main() {
    let f = bar(foo);
    let x = unsafe { f(2) };
    assert_eq!(x, 44);
}

// ferrocene-annotations: fls_xztr1kebz8bo
// Function Pointer Types
