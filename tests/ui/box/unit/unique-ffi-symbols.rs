//@ run-pass
// We used to have a __rust_abi shim that resulted in duplicated symbols
// whenever the item path wasn't enough to disambiguate between them.
fn main() {
    let a = {
        extern "C" fn good() -> i32 { return 0; }
        good as extern "C" fn() -> i32
    };
    let b = {
        extern "C" fn good() -> i32 { return 5; }
        good as extern "C" fn() -> i32
    };

    assert!(a != b);
    assert_eq!((a(), b()), (0, 5));
}

// ferrocene-annotations: fls_yztwtek0y34v
// External Functions
//
// ferrocene-annotations: fls_1qhsun1vyarz
// Type Cast Expressions
//
// ferrocene-annotations: fls_dw33yt5g6m0k
// Type Coercion
//
// ferrocene-annotations: fls_hbbek3z4wtcs
// Function Types
