//@ run-pass
trait Equal {
    fn equal(&self, other: &Self) -> bool;
    fn equals<T,U>(&self, this: &T, that: &T, x: &U, y: &U) -> bool
            where T: Eq, U: Eq;
}

impl<T> Equal for T where T: Eq {
    fn equal(&self, other: &T) -> bool {
        self == other
    }
    fn equals<U,X>(&self, this: &U, other: &U, x: &X, y: &X) -> bool
            where U: Eq, X: Eq {
        this == other && x == y
    }
}

fn equal<T>(x: &T, y: &T) -> bool where T: Eq {
    x == y
}

fn main() {
    println!("{}", equal(&1, &2));
    println!("{}", equal(&1, &1));
    println!("{}", "hello".equal(&"hello"));
    println!("{}", "hello".equals::<isize,&str>(&1, &1, &"foo", &"bar"));
}

// ferrocene-annotations: fls_85vx1qfa061i
// Traits
//
// ferrocene-annotations: fls_142vncdktbin
// Reference Types
//
// ferrocene-annotations: fls_jeoas4n6su4
// Trait and Lifetime Bounds
//
// ferrocene-annotations: fls_fk2m2irwpeof
// Implementations
//
// ferrocene-annotations: fls_e1pgdlv81vul
// Implementation Conformance
//
// ferrocene-annotations: fls_xa4nbfas01cj
// Call Expressions
//
// ferrocene-annotations: fls_nsvzzbldhq53
// Comparison Expressions
//
// ferrocene-annotations: fls_lstusiu2c8lu
// Lazy Boolean Expressions
//
// ferrocene-annotations: fls_qztk0bkju9u
// Borrow Expression
//
// ferrocene-annotations: fls_a14slch83hzn
// Borrowing
//
// ferrocene-annotations: fls_izdv9i4spokw
// Operator Expressions
