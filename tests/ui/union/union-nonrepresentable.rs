union U { //~ ERROR recursive type `U` has infinite size
    a: u8,
    b: std::mem::ManuallyDrop<U>,
}

fn main() {}

// ferrocene-annotations: fls_3gapgqys3ceb
// Recursive Types
//
// ferrocene-annotations: fls_g1z6bpyjqxkz
// Type Layout
