// Check that we enforce WF conditions related to regions also for
// types in fns.

#![allow(dead_code)]


struct MustBeCopy<T:Copy> {
    t: T
}

struct Foo<T> {
    // needs T: 'static
    x: fn() -> &'static T //~ ERROR E0310
}

struct Bar<T> {
    // needs T: Copy
    x: fn(&'static T) //~ ERROR E0310
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
// ferrocene-annotations: fls_142vncdktbin
// Reference Types
//
// ferrocene-annotations: fls_yqcygq3y6m5j
// Lifetimes
