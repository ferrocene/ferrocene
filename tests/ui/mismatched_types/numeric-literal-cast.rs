fn foo(_: u16) {}
fn foo1(_: f64) {}
fn foo2(_: i32) {}

fn main() {
    foo(1u8);
//~^ ERROR mismatched types
    foo1(2f32);
//~^ ERROR mismatched types
    foo2(3i16);
//~^ ERROR mismatched types
}

// ferrocene-annotations: fls_nrkd5wpi64oo
// Literals
//
// ferrocene-annotations: fls_qwljwqr07slp
// Numeric Types
//
// ferrocene-annotations: fls_b4xporvr64s
// Floating Point Types
//
// ferrocene-annotations: fls_3qnpv2z7yjil
// Integer Types
//
// ferrocene-annotations: fls_qcb1n9c0e5hz
// Functions
//
// ferrocene-annotations: fls_h0dvogc64tfh
// Literal Expressions
