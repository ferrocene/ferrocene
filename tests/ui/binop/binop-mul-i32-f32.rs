fn foo(x: i32, y: f32) -> f32 {
    x * y //~ ERROR cannot multiply `i32` by `f32`
}

fn main() {}

// ferrocene-annotations: fls_1k9mkv7rbezi
// Arithmetic Expressions
//
// ferrocene-annotations: fls_b4xporvr64s
// Floating Point Types
//
// ferrocene-annotations: fls_3qnpv2z7yjil
// Integer Types
//
// ferrocene-annotations: fls_izdv9i4spokw
// Operator Expressions
//
// ferrocene-annotations: fls_zfibijmf8qe1
// Arithmetic Overflow
