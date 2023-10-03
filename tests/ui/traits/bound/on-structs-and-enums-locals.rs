trait Trait {
    fn dummy(&self) { }
}

struct Foo<T:Trait> {
    x: T,
}

fn main() {
    let foo = Foo {
        x: 3
    //~^ ERROR E0277
    };

    let baz: Foo<usize> = loop { };
    //~^ ERROR E0277
}

// ferrocene-annotations: fls_jeoas4n6su4
// Trait and Lifetime Bounds
//
// ferrocene-annotations: fls_vhpwge5123cm
// Generic Parameters
//
// ferrocene-annotations: fls_utuu8mdbuyxm
// Generic Arguments
//
// ferrocene-annotations: fls_i7g2n7hfg3ch
// Generic Conformance
//
// ferrocene-annotations: fls_9ucqbbd0s2yo
// Struct Types
