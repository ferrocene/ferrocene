//@ edition:2018

// The local `use` suggestion should start with `crate::` (but the
// standard-library suggestions should not, obviously).

mod plumbing {
    pub struct Drain;
}

fn main() {
    let _d = Drain {};
    //~^ ERROR cannot find struct, variant or union type `Drain` in this scope
}

// ferrocene-annotations: fls_e9hwvqsib5d5
// Modules
//
// ferrocene-annotations: fls_jdknpu3kf865
// Visibility
