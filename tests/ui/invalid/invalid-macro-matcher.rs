#![allow(unused_macros)]

macro_rules! invalid {
    _ => (); //~ ERROR invalid macro matcher
}

fn main() {
}

// ferrocene-annotations: fls_xa7lp0zg1ol2
// Declarative Macros
