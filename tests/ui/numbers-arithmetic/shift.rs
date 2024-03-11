//@ run-pass
#![allow(non_upper_case_globals)]
#![allow(overflowing_literals)]

// Testing shifts for various combinations of integers
// Issue #1570


pub fn main() {
    test_misc();
    test_expr();
    test_const();
}

fn test_misc() {
    assert_eq!(1 << 1 << 1 << 1 << 1 << 1, 32);
}

fn test_expr() {
    let v10 = 10 as usize;
    let v4 = 4 as u8;
    let v2 = 2 as u8;
    assert_eq!(v10 >> v2 as usize, v2 as usize);
    assert_eq!(v10 << v4 as usize, 160 as usize);

    let v10 = 10 as u8;
    let v4 = 4 as usize;
    let v2 = 2 as usize;
    assert_eq!(v10 >> v2 as usize, v2 as u8);
    assert_eq!(v10 << v4 as usize, 160 as u8);

    let v10 = 10 as isize;
    let v4 = 4 as i8;
    let v2 = 2 as i8;
    assert_eq!(v10 >> v2 as usize, v2 as isize);
    assert_eq!(v10 << v4 as usize, 160 as isize);

    let v10 = 10 as i8;
    let v4 = 4 as isize;
    let v2 = 2 as isize;
    assert_eq!(v10 >> v2 as usize, v2 as i8);
    assert_eq!(v10 << v4 as usize, 160 as i8);

    let v10 = 10 as usize;
    let v4 = 4 as isize;
    let v2 = 2 as isize;
    assert_eq!(v10 >> v2 as usize, v2 as usize);
    assert_eq!(v10 << v4 as usize, 160 as usize);
}

fn test_const() {
    static r1_1: usize = 10_usize >> 2_usize;
    static r2_1: usize = 10_usize << 4_usize;
    assert_eq!(r1_1, 2 as usize);
    assert_eq!(r2_1, 160 as usize);

    static r1_2: u8 = 10u8 >> 2_usize;
    static r2_2: u8 = 10u8 << 4_usize;
    assert_eq!(r1_2, 2 as u8);
    assert_eq!(r2_2, 160 as u8);

    static r1_3: isize = 10 >> 2_usize;
    static r2_3: isize = 10 << 4_usize;
    assert_eq!(r1_3, 2 as isize);
    assert_eq!(r2_3, 160 as isize);

    static r1_4: i8 = 10i8 >> 2_usize;
    static r2_4: i8 = 10i8 << 4_usize;
    assert_eq!(r1_4, 2 as i8);
    assert_eq!(r2_4, 160 as i8);

    static r1_5: usize = 10_usize >> 2_usize;
    static r2_5: usize = 10_usize << 4_usize;
    assert_eq!(r1_5, 2 as usize);
    assert_eq!(r2_5, 160 as usize);
}

// ferrocene-annotations: fls_abp6tjbz8tpn
// Bit Expressions
//
// ferrocene-annotations: fls_2ed4axpsy9u0
// Integer Literals
//
// ferrocene-annotations: fls_3qnpv2z7yjil
// Integer Types
//
// ferrocene-annotations: fls_1qhsun1vyarz
// Type Cast Expressions
//
// ferrocene-annotations: fls_dw33yt5g6m0k
// Type Coercion
//
// ferrocene-annotations: fls_h0dvogc64tfh
// Literal Expressions
//
// ferrocene-annotations: fls_e7zgqroy2qxn
// Value Expressions
//
// ferrocene-annotations: fls_izdv9i4spokw
// Operator Expressions
