//@ check-pass
#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

trait If<const COND: bool> {}
impl If<true> for () {}

trait IsZero<const N: u8> {
    type Answer;
}

struct True;
struct False;

impl<const N: u8> IsZero<N> for ()
where (): If<{N == 0}> {
    type Answer = True;
}

trait Foobar<const N: u8> {}

impl<const N: u8> Foobar<N> for ()
where (): IsZero<N, Answer = True> {}

impl<const N: u8> Foobar<N> for ()
where (): IsZero<N, Answer = False> {}

fn main() {}

// ferrocene-annotations: fls_fk2m2irwpeof
// Implementations

// ferrocene-annotations: fls_85vx1qfa061i
// Traits
