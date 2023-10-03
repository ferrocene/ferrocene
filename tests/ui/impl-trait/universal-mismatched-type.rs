use std::fmt::Debug;

fn foo(x: impl Debug) -> String {
    x //~ ERROR mismatched types
}

fn main() { }

// ferrocene-annotations: fls_exe4zodlwfez
// Type Unification
