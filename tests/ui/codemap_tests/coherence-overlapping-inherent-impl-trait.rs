#![allow(dead_code)]

trait C {}
impl dyn C { fn f() {} } //~ ERROR duplicate
impl dyn C { fn f() {} }
fn main() { }

// ferrocene-annotations: fls_ydmnb7qnmzzq
// Shadowing
//
// ferrocene-annotations: fls_l21tjqjkkaa0
// Associated Items
//
// ferrocene-annotations: fls_fk2m2irwpeof
// Implementations
