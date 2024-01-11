union U {
    a: str,
    //~^ ERROR the size for values of type
    //~| ERROR field must implement `Copy`

    b: u8,
}

union W {
    a: u8,
    b: str,
    //~^ ERROR the size for values of type
    //~| ERROR field must implement `Copy`
}

fn main() {}

// ferrocene-annotations: fls_g1z6bpyjqxkz
// Type Layout
