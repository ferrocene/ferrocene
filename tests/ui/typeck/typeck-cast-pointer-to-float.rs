fn main() {
    let x : i16 = 22;
    ((&x) as *const i16) as f32;
    //~^ ERROR casting `*const i16` as `f32` is invalid
}

// ferrocene-annotations: fls_1qhsun1vyarz
// Type Cast Expressions
//
// ferrocene-annotations: fls_dw33yt5g6m0k
// Type Coercion
//
// ferrocene-annotations: fls_ppd1xwve3tr7
// Raw Pointer Types
