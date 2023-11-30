// Check that we test WF conditions for fn arguments. Because the
// current code is so goofy, this is only a warning for now.


#![allow(dead_code)]
#![allow(unused_variables)]

trait Bar<T:Eq+?Sized> { }

trait Foo {
    fn bar<A>(&self) where A: Bar<Self> {
        //~^ ERROR E0277
        //
        // Here, Eq ought to be implemented.
    }
}


fn main() { }

// ferrocene-annotations: fls_9ucqbbd0s2yo
// Struct Types
//
// ferrocene-annotations: fls_vhpwge5123cm
// Generic Parameters
//
// ferrocene-annotations: fls_jeoas4n6su4
// Trait and Lifetime Bounds
//
// ferrocene-annotations: fls_7nv8ualeaqe3
// Where Clauses
