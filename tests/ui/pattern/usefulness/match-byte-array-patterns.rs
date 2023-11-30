#![deny(unreachable_patterns)]

fn main() {
    let buf = &[0, 1, 2, 3];

    match buf {
        b"AAAA" => {},
        &[0x41, 0x41, 0x41, 0x41] => {} //~ ERROR unreachable pattern
        _ => {}
    }

    match buf {
        &[0x41, 0x41, 0x41, 0x41] => {}
        b"AAAA" => {}, //~ ERROR unreachable pattern
        _ => {}
    }

    match buf {
        &[_, 0x41, 0x41, 0x41] => {},
        b"AAAA" => {}, //~ ERROR unreachable pattern
        _ => {}
    }

    match buf {
        &[0x41, .., 0x41] => {}
        b"AAAA" => {}, //~ ERROR unreachable pattern
        _ => {}
    }

    let buf: &[u8] = buf;

    match buf {
        b"AAAA" => {},
        &[0x41, 0x41, 0x41, 0x41] => {} //~ ERROR unreachable pattern
        _ => {}
    }

    match buf {
        &[0x41, 0x41, 0x41, 0x41] => {}
        b"AAAA" => {}, //~ ERROR unreachable pattern
        _ => {}
    }

    match buf {
        &[_, 0x41, 0x41, 0x41] => {},
        b"AAAA" => {}, //~ ERROR unreachable pattern
        _ => {}
    }

    match buf {
        &[0x41, .., 0x41] => {}
        b"AAAA" => {}, //~ ERROR unreachable pattern
        _ => {}
    }
}

// ferrocene-annotations: fls_xinykul167l
// Array Expressions
//
// ferrocene-annotations: fls_qztk0bkju9u
// Borrow Expression
//
// ferrocene-annotations: fls_a14slch83hzn
// Borrowing
//
// ferrocene-annotations: fls_lv7w7aalpwm5
// Type Inference
//
// ferrocene-annotations: fls_e5td0fa92fay
// Match Expressions
//
// ferrocene-annotations: fls_fqaffyrjob7v
// Byte String Literals
//
// ferrocene-annotations: fls_msbaxfc09vkk
// Simple Byte String Literals
//
// ferrocene-annotations: fls_vpbikb73dw4k
// Slice Types
//
// ferrocene-annotations: fls_2krxnq8q9ef1
// Literal Patterns
//
// ferrocene-annotations: fls_azzf1llv3wf
// Literal Pattern Matching
//
// ferrocene-annotations: fls_qte70mgzpras
// Slice Patterns
//
// ferrocene-annotations: fls_57ic33pwdvp3
// Slice Pattern Matching
//
// ferrocene-annotations: fls_qfsfnql1t7m
// Underscore Patterns
//
// ferrocene-annotations: fls_yc4xm4hrfyw7
// Underscore Pattern Matching
//
// ferrocene-annotations: fls_7wpgnp4kjq82
// Rest Patterns
