pub enum T {
    T1(()),
    T2(()),
}

pub enum V {
    V1(isize),
    V2(bool),
}

fn main() {
    match (T::T1(()), V::V2(true)) {
        //~^ ERROR non-exhaustive patterns: `(T::T1(()), V::V2(_))` and `(T::T2(()), V::V1(_))` not covered
        (T::T1(()), V::V1(i)) => (),
        (T::T2(()), V::V2(b)) => (),
    }
}

// ferrocene-annotations: fls_jdknpu3kf865
// Visibility
//
// ferrocene-annotations: fls_szibmtfv117b
// Enum Types
//
// ferrocene-annotations: fls_e5td0fa92fay
// Match Expressions
//
// ferrocene-annotations: fls_urbr5rg9206v
// Tuple Patterns
//
// ferrocene-annotations: fls_rce8bb7nz2jy
// Tuple Pattern Matching
//
// ferrocene-annotations: fls_uloyjbaso8pz
// Path Patterns
//
// ferrocene-annotations: fls_d44aflefat88
// Path Pattern Matching
