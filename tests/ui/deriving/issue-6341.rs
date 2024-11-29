//@ check-pass

#[derive(PartialEq)]
struct A { x: usize }

impl Drop for A {
    fn drop(&mut self) {}
}

pub fn main() {}

// ferrocene-annotations: fls_nsvzzbldhq53
// Comparison Expressions
//
// ferrocene-annotations: fls_u2mzjgiwbkz0
// Destructors
