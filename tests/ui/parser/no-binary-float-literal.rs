fn main() {
    0b101010f64;
    //~^ ERROR binary float literal is not supported
    0b101.010;
    //~^ ERROR binary float literal is not supported
    0b101p4f64;
    //~^ ERROR invalid suffix `p4f64` for number literal
}

// ferrocene-annotations: fls_29tlg1vyqay2
// Float Literals
//
// ferrocene-annotations: fls_2ed4axpsy9u0
// Integer Literals
//
// ferrocene-annotations: fls_hv9jtycp0o1y
// Numeric Literals
