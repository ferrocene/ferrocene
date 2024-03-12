//@ check-pass

// This used to ICE because the "if" being unreachable was not handled correctly
fn err() {
    if loop {} {}
}

fn main() {}

// ferrocene-annotations: fls_exe4zodlwfez
// Type Unification
// ferrocene-annotations: fls_mkut7gut49gi
// If Expressions
