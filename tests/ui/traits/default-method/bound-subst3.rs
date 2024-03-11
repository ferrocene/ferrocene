//@ run-pass


trait A {
    fn g<T>(&self, x: T, y: T) -> (T, T) { (x, y) }
}

impl A for isize { }

fn f<T, V: A>(i: V, j: T, k: T) -> (T, T) {
    i.g(j, k)
}

pub fn main () {
    assert_eq!(f(0, 1, 2), (1, 2));
    assert_eq!(f(0, 1u8, 2u8), (1u8, 2u8));
}

// ferrocene-annotations: fls_z7q8kbjwdc7g
// Method Call Expressions
//
// ferrocene-annotations: fls_jeoas4n6su4
// Trait and Lifetime Bounds
//
// ferrocene-annotations: fls_wqazkzle0ix9
// Method Resolution
