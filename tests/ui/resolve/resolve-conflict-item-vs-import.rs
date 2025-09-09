use std::mem::transmute; //~ NOTE previous import of the value `transmute` here

fn transmute() {}
//~^ ERROR the name `transmute` is defined multiple times
//~| NOTE `transmute` redefined here
//~| NOTE `transmute` must be defined only once in the value namespace of this module
fn main() {
}

// ferrocene-annotations: fls_9gprp17h6t1q
// Use Imports
