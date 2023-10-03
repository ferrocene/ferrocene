fn foo<N>(_x: N) {}

fn main() {
    let x_usize: usize = 1;
    let x_u64: u64 = 2;
    let x_u32: u32 = 3;
    let x_u16: u16 = 4;
    let x_u8: u8 = 5;
    let x_isize: isize = 6;
    let x_i64: i64 = 7;
    let x_i32: i32 = 8;
    let x_i16: i16 = 9;
    let x_i8: i8 = 10;
    let x_f64: f64 = 11.0;
    let x_f32: f32 = 12.0;

    foo::<usize>(x_f64); //~ ERROR mismatched types
    foo::<usize>(x_f32); //~ ERROR mismatched types
    foo::<isize>(x_f64); //~ ERROR mismatched types
    foo::<isize>(x_f32); //~ ERROR mismatched types
    foo::<u64>(x_f64); //~ ERROR mismatched types
    foo::<u64>(x_f32); //~ ERROR mismatched types
    foo::<i64>(x_f64); //~ ERROR mismatched types
    foo::<i64>(x_f32); //~ ERROR mismatched types
    foo::<u32>(x_f64); //~ ERROR mismatched types
    foo::<u32>(x_f32); //~ ERROR mismatched types
    foo::<i32>(x_f64); //~ ERROR mismatched types
    foo::<i32>(x_f32); //~ ERROR mismatched types
    foo::<u16>(x_f64); //~ ERROR mismatched types
    foo::<u16>(x_f32); //~ ERROR mismatched types
    foo::<i16>(x_f64); //~ ERROR mismatched types
    foo::<i16>(x_f32); //~ ERROR mismatched types
    foo::<u8>(x_f64); //~ ERROR mismatched types
    foo::<u8>(x_f32); //~ ERROR mismatched types
    foo::<i8>(x_f64); //~ ERROR mismatched types
    foo::<i8>(x_f32); //~ ERROR mismatched types
    foo::<f32>(x_f64); //~ ERROR mismatched types
}

// ferrocene-annotations: fls_29tlg1vyqay2
// Float Literals
//
// ferrocene-annotations: fls_b4xporvr64s
// Floating Point Types
//
// ferrocene-annotations: fls_2ed4axpsy9u0
// Integer Literals
//
// ferrocene-annotations: fls_3qnpv2z7yjil
// Integer Types
//
// ferrocene-annotations: fls_lv7w7aalpwm5
// Type Inference
//
// ferrocene-annotations: fls_exe4zodlwfez
// Type Unification
//
// ferrocene-annotations: fls_h0dvogc64tfh
// Literal Expressions
//
// ferrocene-annotations: fls_e7zgqroy2qxn
// Value Expressions
