fn main() {}

struct X;

impl X {
    fn f(); //~ ERROR associated function in `impl` without body
}

// ferrocene-annotations: fls_qcb1n9c0e5hz
// Functions
//
// ferrocene-annotations: fls_fk2m2irwpeof
// Implementations
