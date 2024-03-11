//@ run-pass



pub fn main() {
    let f = 4.999999999999f64;
    assert!((f > 4.90f64));
    assert!((f < 5.0f64));
    let g = 4.90000000001e-10f64;
    assert!((g > 5e-11f64));
    assert!((g < 5e-9f64));
}

// ferrocene-annotations: fls_nsvzzbldhq53
// Comparison Expressions
//
// ferrocene-annotations: fls_29tlg1vyqay2
// Float Literals
