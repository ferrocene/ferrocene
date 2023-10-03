trait Trait {}

struct Foo<T:Trait> {
    x: T,
}

enum Bar<T:Trait> {
    ABar(isize),
    BBar(T),
    CBar(usize),
}

impl<T> Foo<T> {
//~^ ERROR `T: Trait` is not satisfied
    fn uhoh() {}
}

struct Baz {
    a: Foo<isize>, //~ ERROR E0277
}

enum Boo {
    Quux(Bar<usize>), //~ ERROR E0277
}

struct Badness<U> {
    b: Foo<U>, //~ ERROR E0277
}

enum MoreBadness<V> {
    EvenMoreBadness(Bar<V>), //~ ERROR E0277
}

struct TupleLike(
    Foo<i32>, //~ ERROR E0277
);

enum Enum {
    DictionaryLike { field: Bar<u8> }, //~ ERROR E0277
}

fn main() {
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
//
// ferrocene-annotations: fls_szibmtfv117b
// Enum Types
