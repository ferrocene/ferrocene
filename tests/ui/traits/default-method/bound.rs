//@ run-pass


trait A {
    fn g(&self) -> isize { 10 }
}

impl A for isize { }

fn f<T:A>(i: T) {
    assert_eq!(i.g(), 10);
}

pub fn main () {
    f(0);
}

// ferrocene-annotations: fls_z7q8kbjwdc7g
// Method Call Expressions
//
// ferrocene-annotations: fls_jeoas4n6su4
// Trait and Lifetime Bounds
//
// ferrocene-annotations: fls_wqazkzle0ix9
// Method Resolution
