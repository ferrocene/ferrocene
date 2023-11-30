#![allow(non_camel_case_types)]

mod foo { pub struct bar; }

fn main() {
    let bar = 5;
    //~^ ERROR mismatched types
    use foo::bar;
}

// ferrocene-annotations: fls_9gprp17h6t1q
// Use Imports
//
// ferrocene-annotations: fls_ydmnb7qnmzzq
// Shadowing
//
// ferrocene-annotations: fls_exe4zodlwfez
// Type Unification
