trait Tr : Sized {
    fn test<X>(u: X) -> Self {
        u   //~ ERROR mismatched types
    }
}

fn main() {}

// ferrocene-annotations: fls_vhpwge5123cm
// Generic Parameters
// ferrocene-annotations: fls_exe4zodlwfez
// Type Unification
