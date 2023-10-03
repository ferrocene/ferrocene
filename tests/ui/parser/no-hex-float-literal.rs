fn main() {
    0xABC.Df;
    //~^ ERROR `{integer}` is a primitive type and therefore doesn't have fields
    0x567.89;
    //~^ ERROR hexadecimal float literal is not supported
    0xDEAD.BEEFp-2f;
    //~^ ERROR invalid suffix `f` for float literal
    //~| ERROR `{integer}` is a primitive type and therefore doesn't have fields
}

// ferrocene-annotations: fls_29tlg1vyqay2
// Float Literals
//
// ferrocene-annotations: fls_2ed4axpsy9u0
// Integer Literals
//
// ferrocene-annotations: fls_hv9jtycp0o1y
// Numeric Literals
