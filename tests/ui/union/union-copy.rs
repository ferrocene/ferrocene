#[derive(Clone)]
union U {
    a: u8
}

#[derive(Clone)]
union W {
    a: std::mem::ManuallyDrop<String>
}

impl Copy for U {} // OK
impl Copy for W {} //~ ERROR the trait `Copy` may not be implemented for this type

fn main() {}

// ferrocene-annotations: fls_r6gj1p4gajnq
// Attribute derive
//
// ferrocene-annotations: fls_fk2m2irwpeof
// Implementations
