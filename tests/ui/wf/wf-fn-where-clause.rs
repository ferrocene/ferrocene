// Test that we check where-clauses on fn items.


#![allow(dead_code)]

trait ExtraCopy<T:Copy> { }

fn foo<T,U>() where T: ExtraCopy<U> //~ ERROR E0277
{
}

fn bar() where Vec<dyn Copy>:, {}
//~^ ERROR E0277
//~| ERROR E0038

struct Vec<T> {
    t: T,
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
//
// ferrocene-annotations: fls_qa98qdi42orq
// Trait Object Types
