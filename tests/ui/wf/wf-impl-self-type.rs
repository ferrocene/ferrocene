// Tests that we point at the proper location for an error
// involving the self-type of an impl

trait Foo {}
impl Foo for Option<[u8]> {} //~ ERROR the size for

fn main() {}

// ferrocene-annotations: fls_85vx1qfa061i
// Traits
//
// ferrocene-annotations: fls_46ork6fz5o2e
// Implementation Coherence
//
// ferrocene-annotations: fls_vpbikb73dw4k
// Slice Types
