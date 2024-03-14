trait Deserializer<'a> { }

trait Deserializable {
    fn deserialize_token<'a, D: Deserializer<'a>>(_: D, _: &'a str) -> Self;
}

impl<'a, T: Deserializable> Deserializable for &'a str {
    //~^ ERROR type parameter `T` is not constrained
    fn deserialize_token<D: Deserializer<'a>>(_x: D, _y: &'a str) -> &'a str {
        //~^ ERROR mismatched types
    }
}

fn main() {}

// ferrocene-annotations: fls_85vx1qfa061i
// Traits
//
// ferrocene-annotations: fls_fk2m2irwpeof
// Implementations
//
// ferrocene-annotations: fls_e1pgdlv81vul
// Implementation Conformance
//
// ferrocene-annotations: fls_vhpwge5123cm
// Generic Parameters
//
// ferrocene-annotations: fls_yqcygq3y6m5j
// Lifetimes
//
// ferrocene-annotations: fls_ikfvbeewame7
// Subtyping and Variance
