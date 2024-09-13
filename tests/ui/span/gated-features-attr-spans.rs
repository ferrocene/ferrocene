#[repr(simd)] //~ ERROR are experimental
struct Coord {
    v: [u32; 2],
}

fn main() {}

// ferrocene-annotations: fls_aibb2quva4mn
// Attribute repr
