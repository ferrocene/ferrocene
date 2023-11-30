/* Checks whether primitive type names are formatted correctly in the
 * error messages about mismatched types (#84976).
 */

fn foo(length: &u32) -> i32 {
    0
}

fn bar(length: &f32) -> f64 {
    0.0
}

fn main() {
    let mut length = 0;
    length = { foo(&length) };
    //~^ ERROR mismatched types [E0308]
    length = foo(&length);
    //~^ ERROR mismatched types [E0308]

    let mut float_length = 0.0;
    float_length = { bar(&float_length) };
    //~^ ERROR mismatched types [E0308]
    float_length = bar(&float_length);
    //~^ ERROR mismatched types [E0308]
}

// ferrocene-annotations: fls_qwljwqr07slp
// Numeric Types
//
// ferrocene-annotations: fls_b4xporvr64s
// Floating Point Types
//
// ferrocene-annotations: fls_3qnpv2z7yjil
// Integer Types
//
// ferrocene-annotations: fls_nrkd5wpi64oo
// Literals
//
// ferrocene-annotations: fls_h0dvogc64tfh
// Literal Expressions
