//@ edition: 2015
extern crate core;
pub use core as reexported_core; //~ ERROR `core` is private and cannot be re-exported
                                 //~^ WARN this was previously accepted

mod foo1 {
    extern crate core;
}

mod foo2 {
    use foo1::core; //~ ERROR crate import `core` is private
    pub mod bar {
        extern crate core;
    }
}

mod baz {
    pub use foo2::bar::core; //~ ERROR crate import `core` is private
}

fn main() {}

// ferrocene-annotations: fls_maw4u1o8q37u
// Crates
