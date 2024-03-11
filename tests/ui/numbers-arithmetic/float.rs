//@ run-pass
pub fn main() {
    let pi = 3.1415927f64;
    println!("{}", -pi * (pi + 2.0 / pi) - pi * 5.0);
    if pi == 5.0 || pi < 10.0 || pi <= 2.0 || pi != 22.0 / 7.0 || pi >= 10.0
           || pi > 1.0 {
        println!("yes");
    }
}

// ferrocene-annotations: fls_nsvzzbldhq53
// Comparison Expressions
//
// ferrocene-annotations: fls_29tlg1vyqay2
// Float Literals
//
// ferrocene-annotations: fls_h0dvogc64tfh
// Literal Expressions
//
// ferrocene-annotations: fls_e7zgqroy2qxn
// Value Expressions
//
// ferrocene-annotations: fls_izdv9i4spokw
// Operator Expressions
