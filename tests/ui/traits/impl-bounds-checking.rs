pub trait Clone2 {
    fn clone(&self) -> Self;
}


trait Getter<T: Clone2> {
    fn get(&self) -> T;
}

impl Getter<isize> for isize { //~ ERROR `isize: Clone2` is not satisfied
    fn get(&self) -> isize { *self }
}

fn main() { }

// ferrocene-annotations: fls_jeoas4n6su4
// Trait and Lifetime Bounds
