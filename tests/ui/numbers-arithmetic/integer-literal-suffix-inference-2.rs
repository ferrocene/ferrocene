//@ run-pass

fn foo(_: *const ()) {}

fn main() {
    let a = 3;
    foo(&a as *const _ as *const ());
}

// ferrocene-annotations: fls_2ed4axpsy9u0
// Integer Literals
//
// ferrocene-annotations: fls_1qhsun1vyarz
// Type Cast Expressions
//
// ferrocene-annotations: fls_dw33yt5g6m0k
// Type Coercion
//
// ferrocene-annotations: fls_lv7w7aalpwm5
// Type Inference
//
// ferrocene-annotations: fls_izdv9i4spokw
// Operator Expressions
