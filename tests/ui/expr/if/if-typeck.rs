//@ error-pattern:mismatched types
// issue #513

fn f() { }

fn main() {

    // f is not a bool
    if f { }
}

// ferrocene-annotations: fls_exe4zodlwfez
// Type Unification
// ferrocene-annotations: fls_mkut7gut49gi
// If Expressions
