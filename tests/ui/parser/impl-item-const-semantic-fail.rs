fn main() {}

struct X;

impl X {
    const Y: u8; //~ ERROR associated constant in `impl` without body
}

// ferrocene-annotations: fls_ixjc5jaamx84
// Constants
//
// ferrocene-annotations: fls_fk2m2irwpeof
// Implementations
