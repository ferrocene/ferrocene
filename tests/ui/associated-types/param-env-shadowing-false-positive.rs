trait Trait {
    type Assoc;
}

impl<T> Trait for T {
    type Assoc = T;
}

fn foo<T: Trait>(x: T::Assoc) -> u32 {
    x //~ ERROR mismatched types
}

fn main() {}
