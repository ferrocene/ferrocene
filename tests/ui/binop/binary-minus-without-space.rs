//@ run-pass
// Check that issue #954 stays fixed


pub fn main() {
    match -1 { -1 => {}, _ => panic!("wat") }
    assert_eq!(1-1, 0);
}

// ferrocene-annotations: fls_wrecura8u5ar
// Negation Expression
//
// ferrocene-annotations: fls_1k9mkv7rbezi
// Arithmetic Expressions
//
// ferrocene-annotations: fls_izdv9i4spokw
// Operator Expressions
//
// ferrocene-annotations: fls_zfibijmf8qe1
// Arithmetic Overflow
