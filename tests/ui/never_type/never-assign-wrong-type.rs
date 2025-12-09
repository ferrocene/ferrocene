// Test that we can't use another type in place of !

#![feature(never_type)]

fn main() {
    let x: ! = "hello"; //~ ERROR mismatched types
}

// ferrocene-annotations: fls_exe4zodlwfez
// Type Unification
