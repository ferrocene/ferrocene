trait Foo {
}



impl<T:Copy> Foo for T {
}

fn take_param<T:Foo>(foo: &T) { }

fn main() {
    let x: Box<_> = Box::new(3);
    take_param(&x);
    //~^ ERROR the trait bound `Box<{integer}>: Foo` is not satisfied
}

// ferrocene-annotations: fls_utuu8mdbuyxm
// Generic Arguments
//
// ferrocene-annotations: fls_i7g2n7hfg3ch
// Generic Conformance
//
// ferrocene-annotations: fls_vhpwge5123cm
// Generic Parameters
