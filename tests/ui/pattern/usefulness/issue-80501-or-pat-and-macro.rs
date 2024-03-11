//@ check-pass
#![deny(unreachable_patterns)]
pub enum TypeCtor {
    Slice,
    Array,
}

pub struct ApplicationTy(TypeCtor);

macro_rules! ty_app {
    ($ctor:pat) => {
        ApplicationTy($ctor)
    };
}

fn _foo(ty: ApplicationTy) {
    match ty {
        ty_app!(TypeCtor::Array) | ty_app!(TypeCtor::Slice) => {}
    }

    // same as above, with the macro expanded
    match ty {
        ApplicationTy(TypeCtor::Array) | ApplicationTy(TypeCtor::Slice) => {}
    }
}

fn main() {}

// ferrocene-annotations: fls_jdknpu3kf865
// Visibility
//
// ferrocene-annotations: fls_szibmtfv117b
// Enum Types
//
// ferrocene-annotations: fls_9ucqbbd0s2yo
// Struct Types
//
// ferrocene-annotations: fls_e5td0fa92fay
// Match Expressions
//
// ferrocene-annotations: fls_uloyjbaso8pz
// Path Patterns
//
// ferrocene-annotations: fls_d44aflefat88
// Path Pattern Matching
