//@ run-pass
fn main() {
    let &ref a = &[0i32] as &[_];
    assert_eq!(a, &[0i32] as &[_]);

    let &ref a = "hello";
    assert_eq!(a, "hello");

    match "foo" {
        "fool" => unreachable!(),
        "foo" => {},
        ref _x => unreachable!()
    }
}

// ferrocene-annotations: fls_142vncdktbin
// Reference Types
//
// ferrocene-annotations: fls_d2sc9hl3v0mk
// Reference Patterns
//
//ferrocene-annotations: fls_org6hqv397fp
// Reference Pattern Matching
//
// ferrocene-annotations: fls_e5td0fa92fay
// Match Expressions
//
// ferrocene-annotations: fls_boyhlu5srp6u
// String Literals
//
// ferrocene-annotations: fls_hucd52suu6it
// Simple String Literals
//
// ferrocene-annotations: fls_7bxv8lybxm18
// Identifier Patterns
//
// ferrocene-annotations: fls_vnai6ag4qrdb
// Identifier Pattern Matching
