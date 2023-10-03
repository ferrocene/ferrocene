#![deny(unreachable_patterns)]

fn main() {
    let s = &[0x00; 4][..]; //Slice of any value
    const MAGIC_TEST: &[u8] = b"TEST"; //Const slice to pattern match with
    match s {
        MAGIC_TEST => (),
        [0x00, 0x00, 0x00, 0x00] => (),
        [84, 69, 83, 84] => (), //~ ERROR unreachable pattern
        _ => (),
    }
    match s {
        [0x00, 0x00, 0x00, 0x00] => (),
        MAGIC_TEST => (),
        [84, 69, 83, 84] => (), //~ ERROR unreachable pattern
        _ => (),
    }
    match s {
        [0x00, 0x00, 0x00, 0x00] => (),
        [84, 69, 83, 84] => (),
        MAGIC_TEST => (), //~ ERROR unreachable pattern
        _ => (),
    }
    const FOO: [u8; 1] = [4];
    match [99] {
        [0x00] => (),
        [4] => (),
        FOO => (), //~ ERROR unreachable pattern
        _ => (),
    }
    const BAR: &[u8; 1] = &[4];
    match &[99] {
        [0x00] => (),
        [4] => (),
        BAR => (), //~ ERROR unreachable pattern
        b"a" => (),
        _ => (),
    }

    const BOO: &[u8; 0] = &[];
    match &[] {
        [] => (),
        BOO => (), //~ ERROR unreachable pattern
        b"" => (), //~ ERROR unreachable pattern
        _ => (), //~ ERROR unreachable pattern
    }

    const CONST1: &[bool; 1] = &[true];
    match &[false] {
        CONST1 => {}
        [true] => {} //~ ERROR unreachable pattern
        [false] => {}
    }
}

// ferrocene-annotations: fls_nrkd5wpi64oo
// Literals
//
// ferrocene-annotations: fls_hv9jtycp0o1y
// Numeric Literals
//
// ferrocene-annotations: fls_2ed4axpsy9u0
// Integer Literals
//
// ferrocene-annotations: fls_2ifjqwnw03ms
// Byte Literals
//
// ferrocene-annotations: fls_fqaffyrjob7v
// Byte String Literals
//
// ferrocene-annotations: fls_msbaxfc09vkk
// Simple Byte String Literals
//
// ferrocene-annotations: fls_jkab8eevzbte
// Boolean Literals
//
// ferrocene-annotations: fls_h0dvogc64tfh
// Literal Expressions
//
// ferrocene-annotations: fls_xinykul167l
// Array Expressions
//
// ferrocene-annotations: fls_qztk0bkju9u
// Borrow Expression
//
// ferrocene-annotations: fls_a14slch83hzn
// Borrowing
//
// ferrocene-annotations: fls_ixjc5jaamx84
// Constants
//
// ferrocene-annotations: fls_vpbikb73dw4k
// Slice Types
//
// ferrocene-annotations: fls_qte70mgzpras
// Slice Patterns
//
// ferrocene-annotations: fls_57ic33pwdvp3
// Slice Pattern Matching
//
// ferrocene-annotations: fls_2krxnq8q9ef1
// Literal Patterns
//
// ferrocene-annotations: fls_azzf1llv3wf
// Literal Pattern Matching
//
// ferrocene-annotations: fls_qfsfnql1t7m
// Underscore Patterns
//
// ferrocene-annotations: fls_yc4xm4hrfyw7
// Underscore Pattern Matching
