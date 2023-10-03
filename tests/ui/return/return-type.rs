struct S<T> {
    t: T,
}

fn foo<T>(x: T) -> S<T> {
    S { t: x }
}

fn bar() {
    foo(4 as usize)
    //~^ ERROR mismatched types
}

fn main() {}

// ferrocene-annotations: fls_exe4zodlwfez
// Type Unification
//
// ferrocene-annotations: fls_hndm19t57wby
// Block Expressions
//
// ferrocene-annotations: fls_lv7w7aalpwm5
// Type Inference
