#[repr(simd)] //~ ERROR are experimental
struct Coord {
    x: u32,
    y: u32,
}

fn main() {}

// ferrocene-annotations: fls_aibb2quva4mn
// Attribute repr
