//@ run-pass


trait A<T> {
    fn g(&self, x: T) -> T { x }
}

impl A<isize> for isize { }

fn f<T, V: A<T>>(i: V, j: T) -> T {
    i.g(j)
}

pub fn main () {
    assert_eq!(f(0, 2), 2);
}

// ferrocene-annotations: fls_z7q8kbjwdc7g
// Method Call Expressions
//
// ferrocene-annotations: fls_jeoas4n6su4
// Trait and Lifetime Bounds
//
// ferrocene-annotations: fls_wqazkzle0ix9
// Method Resolution
