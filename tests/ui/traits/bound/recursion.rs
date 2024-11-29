//@ check-pass

trait I { fn i(&self) -> Self; }

trait A<T:I> {
    fn id(x:T) -> T { x.i() }
}

trait J<T> { fn j(&self) -> T; }

trait B<T:J<T>> {
    fn id(x:T) -> T { x.j() }
}

trait C {
    fn id<T:J<T>>(x:T) -> T { x.j() }
}

pub fn main() { }

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
// ferrocene-annotations: fls_3gapgqys3ceb
// Recursive Types
