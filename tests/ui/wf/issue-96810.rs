struct S<T: Tr>(T::Assoc);

trait Tr {
    type Assoc;
}

struct Hoge<K> {
    s: S<K>, //~ ERROR the trait bound `K: Tr` is not satisfied
    a: u32,
}

fn main() {}

// ferrocene-annotations: fls_85vx1qfa061i
// Traits
//
// ferrocene-annotations: fls_9ucqbbd0s2yo
// Struct Types
//
// ferrocene-annotations: fls_vhpwge5123cm
// Generic Parameters
