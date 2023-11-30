// Check that we require that associated types in an impl are well-formed.



pub trait Foo<'a> {
    type Bar;
}

impl<'a, T> Foo<'a> for T {
    type Bar = &'a T; //~ ERROR E0309
}


fn main() { }

// ferrocene-annotations: fls_85vx1qfa061i
// Traits
//
// ferrocene-annotations: fls_vhpwge5123cm
// Generic Parameters
//
// ferrocene-annotations: fls_yqcygq3y6m5j
// Lifetimes
//
// ferrocene-annotations: fls_l21tjqjkkaa0
// Associated Items
