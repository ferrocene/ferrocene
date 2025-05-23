//@ check-pass
//@ compile-flags: -Znext-solver

pub trait A {}
pub trait B: A {}

pub trait Mirror {
    type Assoc: ?Sized;
}
impl<T: ?Sized> Mirror for T {
    type Assoc = T;
}

pub fn foo<'a>(x: &'a <dyn B + 'static as Mirror>::Assoc) -> &'a (dyn A + 'static) {
    x
}

fn main() {}

// ferrocene-annotations: fls_dw33yt5g6m0k
// Type Coercion
