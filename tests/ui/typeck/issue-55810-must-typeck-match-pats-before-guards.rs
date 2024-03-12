//@ check-pass

// rust-lang/rust#55810: types for a binding in a match arm can be
// inferred from arms that come later in the match.

struct S;

impl S {
    fn method(&self) -> bool {
        unimplemented!()
    }
}

fn get<T>() -> T {
    unimplemented!()
}

fn main() {
    match get() {
        x if x.method() => {}
        &S => {}
    }
}

// ferrocene-annotations: fls_e5td0fa92fay
// Match Expressions
//
// ferrocene-annotations: fls_lv7w7aalpwm5
// Type Inference
//
// ferrocene-annotations: fls_exe4zodlwfez
// Type Unification
