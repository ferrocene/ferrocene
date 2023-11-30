// Check that we enforce WF conditions also for where clauses in fn items.


#![allow(dead_code)]

trait MustBeCopy<T:Copy> {
}

fn bar<T,U>()
    where T: MustBeCopy<U> //~ ERROR E0277
{
}


fn main() { }

// ferrocene-annotations: fls_85vx1qfa061i
// Traits
//
// ferrocene-annotations: fls_vhpwge5123cm
// Generic Parameters
//
// ferrocene-annotations: fls_7nv8ualeaqe3
// Where Clauses
//
// ferrocene-annotations: fls_jeoas4n6su4
// Trait and Lifetime Bounds
