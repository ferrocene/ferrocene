//@ aux-build:two_macros.rs

#[macro_use()]
//~^ WARN unused attribute
extern crate two_macros;

pub fn main() {
    macro_two!();
    //~^ ERROR cannot find macro
}

// ferrocene-annotations: fls_qxjy0f758x5s
// Attribute macro_use
