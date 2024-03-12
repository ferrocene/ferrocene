//@ run-pass


// Tests that we can call a function bounded over a supertrait from
// a default method

fn require_y<T: Y>(x: T) -> isize { x.y() }

trait Y {
    fn y(self) -> isize;
}


trait Z: Y + Sized {
    fn x(self) -> isize {
        require_y(self)
    }
}

impl Y for isize {
    fn y(self) -> isize { self }
}

impl Z for isize {}

pub fn main() {
    assert_eq!(12.x(), 12);
}

// ferrocene-annotations: fls_z7q8kbjwdc7g
// Method Call Expressions
//
// ferrocene-annotations: fls_jeoas4n6su4
// Trait and Lifetime Bounds
//
// ferrocene-annotations: fls_ikfvbeewame7
// Subtyping and Variance
//
// ferrocene-annotations: fls_wqazkzle0ix9
// Method Resolution
