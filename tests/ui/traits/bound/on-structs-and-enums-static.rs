trait Trait {
    fn dummy(&self) { }
}

struct Foo<T:Trait> {
    x: T,
}

static X: Foo<usize> = Foo {
//~^ ERROR E0277
//~| ERROR E0277
    x: 1, //~ ERROR: E0277
};

fn main() {
}

// ferrocene-annotations: fls_xdvdl2ssnhlo
// Statics
//
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
