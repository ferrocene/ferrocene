//@ run-fail
//@ error-pattern:attempt to calculate the remainder with a divisor of zero
//@ needs-subprocess

#[allow(unconditional_panic)]
fn main() {
    let y = 0;
    let _z = 1 % y;
}

// ferrocene-annotations: fls_1k9mkv7rbezi
// Arithmetic Expressions
//
// ferrocene-annotations: fls_izdv9i4spokw
// Operator Expressions
//
// ferrocene-annotations: fls_zfibijmf8qe1
// Arithmetic Overflow
