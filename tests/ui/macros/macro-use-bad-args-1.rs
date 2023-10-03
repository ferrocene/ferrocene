#![no_std]

#[macro_use(foo(bar))]  //~ ERROR bad macro import
extern crate std;

fn main() {}

// ferrocene-annotations: fls_qxjy0f758x5s
// Attribute macro_use
