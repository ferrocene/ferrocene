//@ run-pass

const X1: &'static [u8] = &[b'1'];
const X2: &'static [u8] = b"1";
const X3: &'static [u8; 1] = &[b'1'];
const X4: &'static [u8; 1] = b"1";

static Y1: u8 = X1[0];
static Y2: u8 = X2[0];
static Y3: u8 = X3[0];
static Y4: u8 = X4[0];

fn main() {
    assert_eq!(Y1, Y2);
    assert_eq!(Y1, Y3);
    assert_eq!(Y1, Y4);
}

// ferrocene-annotations: fls_ixjc5jaamx84
// Constants
//
// ferrocene-annotations: fls_142vncdktbin
// Reference Types
//
// ferrocene-annotations: fls_yqcygq3y6m5j
// Lifetimes
//
// ferrocene-annotations: fls_vpbikb73dw4k
// Slice Types
//
// ferrocene-annotations: fls_2ifjqwnw03ms
// Byte Literals
//
// ferrocene-annotations: fls_msbaxfc09vkk
// Simple Byte String Literals
//
// ferrocene-annotations: fls_h0dvogc64tfh
// Literal Expressions
//
// ferrocene-annotations: fls_94a8v54bufn8
// Values
//
// ferrocene-annotations: fls_e7zgqroy2qxn
// Value Expressions
