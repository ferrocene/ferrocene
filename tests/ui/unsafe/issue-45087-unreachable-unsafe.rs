// Verify that unreachable code undergoes unsafety checks.
// revisions: mir thir
// [thir]compile-flags: -Z thir-unsafeck

fn main() {
    return;
    *(1 as *mut u32) = 42;
    //~^ ERROR dereference of raw pointer is unsafe
}

fn panic() -> ! {
    panic!();
}

fn f(a: *mut u32) {
    panic();
    *a = 1;
    //~^ ERROR dereference of raw pointer is unsafe
}

enum Void {}

fn uninhabited() -> Void {
    panic!();
}

fn g(b: *mut u32) {
    uninhabited();
    *b = 1;
    //~^ ERROR dereference of raw pointer is unsafe
}

// ferrocene-annotations: fls_5cm4gkt55hjh
// Dereference Expression
//
// ferrocene-annotations: fls_1qhsun1vyarz
// Type Cast Expressions
//
// ferrocene-annotations: fls_dw33yt5g6m0k
// Type Coercion
//
// ferrocene-annotations: fls_ppd1xwve3tr7
// Raw Pointer Types
