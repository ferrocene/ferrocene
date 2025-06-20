//@ only-x86_64
//@ stderr-per-bitwidth
//@ dont-require-annotations: NOTE

#[repr(C)]
union Nonsense {
    u: usize,
    int_32_ref: &'static i32,
    uint_8: u8,
    uint_16: u16,
    uint_32: u32,
    uint_64: u64,
    uint_128: u128,
    int_8: i8,
    int_16: i16,
    int_32: i32,
    int_64: i64,
    int_128: i128,
    float_32: f32,
    float_64: f64,
    truthy_falsey: bool,
    character: char,
    stringy: &'static str,
}

fn main() {
    const I32_REF_USIZE_UNION: usize = unsafe { Nonsense { int_32_ref: &3 }.u };
    //~^ ERROR unable to turn pointer into integer

    const I32_REF_U8_UNION: u8 = unsafe { Nonsense { int_32_ref: &3 }.uint_8 };
    //~^ ERROR unable to turn pointer into integer

    const I32_REF_U16_UNION: u16 = unsafe { Nonsense { int_32_ref: &3 }.uint_16 };
    //~^ ERROR unable to turn pointer into integer

    const I32_REF_U32_UNION: u32 = unsafe { Nonsense { int_32_ref: &3 }.uint_32 };
    //~^ ERROR unable to turn pointer into integer

    const I32_REF_U64_UNION: u64 = unsafe { Nonsense { int_32_ref: &3 }.uint_64 };
    //~^ ERROR unable to turn pointer into integer

    const I32_REF_U128_UNION: u128 = unsafe { Nonsense { int_32_ref: &3 }.uint_128 };
    //~^ ERROR uninitialized

    const I32_REF_I8_UNION: i8 = unsafe { Nonsense { int_32_ref: &3 }.int_8 };
    //~^ ERROR unable to turn pointer into integer

    const I32_REF_I16_UNION: i16 = unsafe { Nonsense { int_32_ref: &3 }.int_16 };
    //~^ ERROR unable to turn pointer into integer

    const I32_REF_I32_UNION: i32 = unsafe { Nonsense { int_32_ref: &3 }.int_32 };
    //~^ ERROR unable to turn pointer into integer

    const I32_REF_I64_UNION: i64 = unsafe { Nonsense { int_32_ref: &3 }.int_64 };
    //~^ ERROR unable to turn pointer into integer

    const I32_REF_I128_UNION: i128 = unsafe { Nonsense { int_32_ref: &3 }.int_128 };
    //~^ ERROR uninitialized

    const I32_REF_F32_UNION: f32 = unsafe { Nonsense { int_32_ref: &3 }.float_32 };
    //~^ ERROR unable to turn pointer into integer

    const I32_REF_F64_UNION: f64 = unsafe { Nonsense { int_32_ref: &3 }.float_64 };
    //~^ ERROR unable to turn pointer into integer

    const I32_REF_BOOL_UNION: bool = unsafe { Nonsense { int_32_ref: &3 }.truthy_falsey };
    //~^ ERROR unable to turn pointer into integer

    const I32_REF_CHAR_UNION: char = unsafe { Nonsense { int_32_ref: &3 }.character };
    //~^ ERROR unable to turn pointer into integer

    const STR_U8_UNION: u8 = unsafe { Nonsense { stringy: "3" }.uint_8 };
    //~^ ERROR unable to turn pointer into integer

    const STR_U16_UNION: u16 = unsafe { Nonsense { stringy: "3" }.uint_16 };
    //~^ ERROR unable to turn pointer into integer

    const STR_U32_UNION: u32 = unsafe { Nonsense { stringy: "3" }.uint_32 };
    //~^ ERROR unable to turn pointer into integer

    const STR_U64_UNION: u64 = unsafe { Nonsense { stringy: "3" }.uint_64 };
    //~^ ERROR unable to turn pointer into integer

    const STR_U128_UNION: u128 = unsafe { Nonsense { stringy: "3" }.uint_128 };
    //~^ ERROR unable to turn pointer into integer

    const STR_I8_UNION: i8 = unsafe { Nonsense { stringy: "3" }.int_8 };
    //~^ ERROR unable to turn pointer into integer

    const STR_I16_UNION: i16 = unsafe { Nonsense { stringy: "3" }.int_16 };
    //~^ ERROR unable to turn pointer into integer

    const STR_I32_UNION: i32 = unsafe { Nonsense { stringy: "3" }.int_32 };
    //~^ ERROR unable to turn pointer into integer

    const STR_I64_UNION: i64 = unsafe { Nonsense { stringy: "3" }.int_64 };
    //~^ ERROR unable to turn pointer into integer

    const STR_I128_UNION: i128 = unsafe { Nonsense { stringy: "3" }.int_128 };
    //~^ ERROR unable to turn pointer into integer

    const STR_F32_UNION: f32 = unsafe { Nonsense { stringy: "3" }.float_32 };
    //~^ ERROR unable to turn pointer into integer

    const STR_F64_UNION: f64 = unsafe { Nonsense { stringy: "3" }.float_64 };
    //~^ ERROR unable to turn pointer into integer

    const STR_BOOL_UNION: bool = unsafe { Nonsense { stringy: "3" }.truthy_falsey };
    //~^ ERROR unable to turn pointer into integer

    const STR_CHAR_UNION: char = unsafe { Nonsense { stringy: "3" }.character };
    //~^ ERROR unable to turn pointer into integer
}

// ferrocene-annotations: fls_18k3uajrgq5f
// Field Access Expressions
//
// ferrocene-annotations: fls_8tsynkj2cufj
// Struct Expressions
//
// ferrocene-annotations: fls_fmdn7n7s413d
// Union Type
//
// ferrocene-annotations: fls_9kjpxri0axvg
// Weak Keywords
//
// ferrocene-annotations: fls_6ydylimiv553
// Place Expressions
//
// ferrocene-annotations: fls_xcwfotmq2e5d
// Field Resolution
