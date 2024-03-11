// Regression test for #87549.
//@ incremental

pub struct Table<T, const N: usize>([Option<T>; N]);

impl<'a, T, const N: usize> IntoIterator for &'a Table<T, N> {
    type IntoIter = std::iter::Flatten<std::slice::Iter<'a, T>>; //~ ERROR `&'a T` is not an iterator
    //~^ ERROR `&'a T` is not an iterator
    type Item = &'a T;

    fn into_iter(self) -> Self::IntoIter { //~ ERROR `&'a T` is not an iterator
        unimplemented!()
    }
}
fn main() {}

// ferrocene-annotations: fls_9ucqbbd0s2yo
// Struct Types
//
// ferrocene-annotations: fls_vhpwge5123cm
// Generic Parameters
//
// ferrocene-annotations: fls_uj0kpjwyld60
// Array Types
//
// ferrocene-annotations: fls_fk2m2irwpeof
// Implementations
//
// ferrocene-annotations: fls_e1pgdlv81vul
// Implementation Conformance
//
// ferrocene-annotations: fls_yqcygq3y6m5j
// Lifetimes
//
// ferrocene-annotations: fls_l21tjqjkkaa0
// Associated Items
