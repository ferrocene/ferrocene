pub trait Trait {
    type A;
}

pub type Alias = dyn Trait<A = Self::A>;
//~^ ERROR failed to resolve: `Self`

fn main() {}

// ferrocene-annotations: fls_utuu8mdbuyxm
// Generic Arguments
// ferrocene-annotations: fls_qa98qdi42orq
// Trait Object Types
// ferrocene-annotations: fls_85vx1qfa061i
// Traits
// ferrocene-annotations: fls_l21tjqjkkaa0
// Associated Items
