// Test static calls to make sure that we align the Self and input
// type parameters on a trait correctly.

trait Tr<T> : Sized {
    fn op(_: T) -> Self;
}

trait A:    Tr<Self> {
    fn test<U>(u: U) -> Self {
        Tr::op(u)   //~ ERROR E0277
    }
}

trait B<T>: Tr<T> {
    fn test<U>(u: U) -> Self {
        Tr::op(u)   //~ ERROR E0277
    }
}

fn main() {
}

// ferrocene-annotations: fls_vhpwge5123cm
// Generic Parameters
// ferrocene-annotations: fls_exe4zodlwfez
// Type Unification
