#![deny(single_use_lifetimes)]
#![allow(dead_code)]
#![allow(unused_variables)]

// Test that we DO warn for a lifetime used only once in an inherent method.

struct Foo<'f> {
    data: &'f u32
}

impl<'f> Foo<'f> { //~ ERROR `'f` only used once
    //~^ HELP elide the single-use lifetime
    fn inherent_a<'a>(&self, data: &'a u32) { //~ ERROR `'a` only used once
        //~^ HELP elide the single-use lifetime
    }
}

fn main() { }

// ferrocene-annotations: fls_9ucqbbd0s2yo
// Struct Types
//
// ferrocene-annotations: fls_fk2m2irwpeof
// Implementations
//
// ferrocene-annotations: fls_qcb1n9c0e5hz
// Functions
