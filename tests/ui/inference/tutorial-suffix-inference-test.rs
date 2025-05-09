//@ dont-require-annotations: NOTE

fn main() {
    let x = 3;
    let y: i32 = 3;

    fn identity_u8(n: u8) -> u8 { n }
    fn identity_u16(n: u16) -> u16 { n }

    identity_u8(x);  // after this, `x` is assumed to have type `u8`
    identity_u16(x);
    //~^ ERROR mismatched types
    //~| NOTE expected `u16`, found `u8`
    identity_u16(y);
    //~^ ERROR mismatched types
    //~| NOTE expected `u16`, found `i32`

    let a = 3;

    fn identity_i(n: isize) -> isize { n }

    identity_i(a); // ok
    identity_u16(a);
    //~^ ERROR mismatched types
    //~| NOTE expected `u16`, found `isize`
}

// ferrocene-annotations: fls_2ed4axpsy9u0
// Integer Literals
//
// ferrocene-annotations: fls_hv9jtycp0o1y
// Numeric Literals
//
// ferrocene-annotations: fls_h0dvogc64tfh
// Literal Expressions
//
// ferrocene-annotations: fls_94a8v54bufn8
// Values
//
// ferrocene-annotations: fls_e7zgqroy2qxn
// Value Expressions
