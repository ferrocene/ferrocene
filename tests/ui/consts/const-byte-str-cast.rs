//@ run-pass
#[deny(warnings)]

pub fn main() {
    let _ = b"x" as &[u8];
    let _ = b"y" as &[u8; 1];
    let _ = b"z" as *const u8;
    let _ = "ä" as *const str;
}

// ferrocene-annotations: fls_fqaffyrjob7v
// Byte string literals

// ferrocene-annotations: fls_1qhsun1vyarz
// Type cast expressions
//
// ferrocene-annotations: fls_h0dvogc64tfh
// Literal Expressions
//
// ferrocene-annotations: fls_94a8v54bufn8
// Values
//
// ferrocene-annotations: fls_e7zgqroy2qxn
// Value Expressions
