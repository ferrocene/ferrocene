//@ aux-build:two_macros.rs

#[macro_use(macro_one)]
extern crate two_macros;

pub fn main() {
    macro_two!();
    //~^ ERROR cannot find macro
}

// ferrocene-annotations: fls_qxjy0f758x5s
// Attribute macro_use
//
// ferrocene-annotations: fls_vnvt40pa48n8
// Macro Invocation
