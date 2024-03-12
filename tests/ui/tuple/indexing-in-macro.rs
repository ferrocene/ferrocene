//@ check-pass

macro_rules! m {
    (.$l:literal) => {};
}

m!(.0.0); // OK, `0.0` after a dot is still a float token.

fn main() {}

// ferrocene-annotations: fls_8nzypdu9j3ge
// Metavariables
// ferrocene-annotations: fls_vnvt40pa48n8
// Macro Invocation
//
// ferrocene-annotations: fls_18k3uajrgq5f
// Field Access Expressions
