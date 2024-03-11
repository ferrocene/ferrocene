//@ run-pass

#![allow(dead_code)]

#[repr(u32)]
enum Tag {
    I,
    F,
}

#[repr(C)]
union U {
    i: i32,
    f: f32,
}

#[repr(C)]
struct Value {
    tag: Tag,
    u: U,
}

fn is_zero(v: Value) -> bool {
    unsafe {
        match v {
            Value { tag: Tag::I, u: U { i: 0 } } => true,
            Value { tag: Tag::F, u: U { f: 0.0 } } => true,
            _ => false,
        }
    }
}

union W {
    a: u8,
    b: u8,
}

fn refut(w: W) {
    unsafe {
        match w {
            W { a: 10 } => {
                panic!();
            }
            W { b } => {
                assert_eq!(b, 11);
            }
        }
    }
}

fn main() {
    let v = Value { tag: Tag::I, u: U { i: 1 } };
    assert_eq!(is_zero(v), false);

    let w = W { a: 11 };
    refut(w);
}

// ferrocene-annotations: fls_aibb2quva4mn
// Attribute repr
//
// ferrocene-annotations: fls_szibmtfv117b
// Enum Types
//
// ferrocene-annotations: fls_xc1hof4qbf6p
// Enum Type Representation
//
// ferrocene-annotations: fls_9ucqbbd0s2yo
// Struct Types
//
// ferrocene-annotations: fls_rjxpof29a3nl
// Struct Type Representation
//
// ferrocene-annotations: fls_8wnyln2nmg4y
// Unsafe Blocks
//
// ferrocene-annotations: fls_jep7p27kaqlp
// Unsafety
//
// ferrocene-annotations: fls_e5td0fa92fay
// Match Expressions
