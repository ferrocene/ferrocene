fn main() {
    let buf = &[0, 1, 2, 3];

    match buf { //~ ERROR non-exhaustive
        b"AAAA" => {}
    }

    let buf: &[u8] = buf;

    match buf { //~ ERROR non-exhaustive
        b"AAAA" => {}
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
