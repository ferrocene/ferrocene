//@ check-pass

fn main() {
    // Test that we can infer the type of binary operands when
    // references are involved, on various types and operators.
    let _: u8 = 0 + 0;
    let _: u8 = 0 + &0;
    let _: u8 = &0 + 0;
    let _: u8 = &0 + &0;

    let _: f32 = 0.0 + 0.0;
    let _: f32 = 0.0 + &0.0;
    let _: f32 = &0.0 + 0.0;
    let _: f32 = &0.0 + &0.0;

    let _: u8 = 0 << 0;
    let _: u8 = 0 << &0;
    let _: u8 = &0 << 0;
    let _: u8 = &0 << &0;

    // Test type inference when variable types are indirectly inferred.
    let a = 22;
    let _: usize = a + &44;

    // When we have no expected type, the types of the operands is the default type.
    let _ = 0 + 0;
    let _ = 0 + &0;
    let _ = &0 + 0;
    let _ = &0 + &0;
}

// ferrocene-annotations: fls_qztk0bkju9u
// Borrow Expression
//
// ferrocene-annotations: fls_v5x85lt5ulva
// References
