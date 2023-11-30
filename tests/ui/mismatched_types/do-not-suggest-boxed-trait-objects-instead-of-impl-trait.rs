struct S;
struct Y;

trait Trait {}

impl Trait for Y {}

fn foo() -> impl Trait {
    if true {
        S
    } else {
        Y //~ ERROR `if` and `else` have incompatible types
    }
}

fn bar() -> impl Trait {
    match true {
        true => S,
        false => Y, //~ ERROR `match` arms have incompatible types
    }
}

fn main() {}

// ferrocene-annotations: fls_e5td0fa92fay
// Match Expressions
//
// ferrocene-annotations: fls_mkut7gut49gi
// If Expressions
//
// ferrocene-annotations: fls_85vx1qfa061i
// Traits
//
// ferrocene-annotations: fls_3xqobbu7wfsf
// Impl Trait Types
