//@ run-pass

#![allow(unused_parens)]

fn foo() { if (return) { } } //~ WARNING unreachable block in `if`

pub fn main() { foo(); }

// ferrocene-annotations: fls_exe4zodlwfez
// Type Unification
// ferrocene-annotations: fls_mkut7gut49gi
// If Expressions
