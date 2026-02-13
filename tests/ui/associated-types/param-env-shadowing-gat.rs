trait Trait {
    type Assoc<T>;
}

impl<T> Trait for T {
    type Assoc<U> = U;
}

fn foo<T: Trait>(x: T::Assoc<T>) -> u32 {
    x //~ ERROR mismatched types
}

fn main() {}
