//@ run-pass

use std::ops::Mul;

#[derive(Copy, Clone)]
pub struct Foo {
    x: f64,
}

impl Mul<Foo> for f64 {
    type Output = Foo;

    fn mul(self, rhs: Foo) -> Foo {
        // intentionally do something that is not *
        Foo { x: self + rhs.x }
    }
}

pub fn main() {
    let f: Foo = Foo { x: 5.0 };
    let val: f64 = 3.0;
    let f2: Foo = val * f;
    assert_eq!(f2.x, 8.0);
}

// ferrocene-annotations: fls_1k9mkv7rbezi
// Arithmetic Expressions
//
// ferrocene-annotations: fls_fk2m2irwpeof
// Implementations
//
// ferrocene-annotations: fls_zfibijmf8qe1
// Arithmetic Overflow
