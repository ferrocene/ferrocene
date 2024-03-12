//@ run-pass

#![allow(unused_parens)]
//@ pretty-expanded FIXME #23616

fn foo() { if (return) { } } //~ WARNING unreachable block in `if`

pub fn main() { foo(); }

// ferrocene-annotations: fls_exe4zodlwfez
// Type Unification
// ferrocene-annotations: fls_mkut7gut49gi
// If Expressions
