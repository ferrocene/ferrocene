//@ run-pass
#![allow(dead_code)]
#![allow(unused_variables)]

trait U {}
trait T<X: U> { fn get(self) -> X; }

trait S2<Y: U> {
    fn m(x: Box<dyn T<Y>+'static>) {}
}

struct St<X: U> {
    f: Box<dyn T<X>+'static>,
}

impl<X: U> St<X> {
    fn blah() {}
}

fn main() {}

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
// ferrocene-annotations: fls_qa98qdi42orq
// Trait Object Types
