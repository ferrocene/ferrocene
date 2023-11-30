// edition:2018
// build-pass
// aux-crate:issue_80074=issue-80074-macro.rs

#[macro_use]
extern crate issue_80074;

fn main() {
    foo!();
}

// ferrocene-annotations: fls_qxjy0f758x5s
// Attribute macro_use
// ferrocene-annotations: fls_9gprp17h6t1q
// Use Imports
// ferrocene-annotations: fls_gklst7joeo33
// External Crates
