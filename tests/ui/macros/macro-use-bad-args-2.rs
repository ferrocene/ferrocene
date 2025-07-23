#![no_std]

#[macro_use(foo="bar")]  //~ ERROR malformed `macro_use` attribute input
extern crate std;

fn main() {}

// ferrocene-annotations: fls_qxjy0f758x5s
// Attribute macro_use
