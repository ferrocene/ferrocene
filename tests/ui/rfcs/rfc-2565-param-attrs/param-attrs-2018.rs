//@ edition:2018

trait Trait2015 { fn foo(#[allow(C)] i32); }
//~^ ERROR expected one of `:`, `@`, or `|`, found `)`

fn main() {}

// ferrocene-annotations: fls_qcb1n9c0e5hz
// Functions
// ferrocene-annotations: fls_l21tjqjkkaa0
// Associated Items
