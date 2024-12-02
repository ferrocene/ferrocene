//@ run-pass

#[derive(Clone)]
#[allow(dead_code)]
struct S<T>(T, ());

pub fn main() {
    let _ = S(1, ()).clone();
}

// ferrocene-annotations: fls_9ucqbbd0s2yo
// Struct Type
//
// ferrocene-annotations: fls_vhpwge5123cm
// Generic Parameters
//
// ferrocene-annotations: fls_k64tfywtn0g8
// Tuple Expressions
