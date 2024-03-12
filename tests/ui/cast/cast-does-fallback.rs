//@ run-pass

pub fn main() {
    // Test that these type check correctly.
    (&42u8 >> 4) as usize;
    (&42u8 << 4) as usize;

    let cap = 512 * 512;
    cap as u8;
    // Assert `cap` did not get inferred to `u8` and overflowed.
    assert_ne!(cap, 0);
}

// ferrocene-annotations: fls_qwljwqr07slp
// Numeric Types
//
// ferrocene-annotations: fls_lv7w7aalpwm5
// Type Inference
//
// ferrocene-annotations: fls_abp6tjbz8tpn
// Bit Expressions
//
// ferrocene-annotations: fls_izdv9i4spokw
// Operator Expressions
