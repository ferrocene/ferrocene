macro_rules! foo {
    ($a:expr) => a; //~ ERROR macro rhs must be delimited
}

fn main() {
    foo!(0); // Check that we report errors at macro definition, not expansion.

    let _: cfg!(FALSE) = (); //~ ERROR non-type macro in type position
}

// ferrocene-annotations: fls_xa7lp0zg1ol2
// Declarative Macros
//
// ferrocene-annotations: fls_9kjpxri0axvg
// Weak Keywords
