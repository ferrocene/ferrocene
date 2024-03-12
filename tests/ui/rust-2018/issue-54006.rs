//@ edition:2018

#![no_std]
#![crate_type = "lib"]

use alloc::vec;
//~^ ERROR unresolved import `alloc`

pub fn foo() {
    let mut xs = vec![];
    xs.push(0);
}

// ferrocene-annotations: fls_9xnaxd7qbakp
// Attribute no_std
//
// ferrocene-annotations: fls_ujig607lmwbm
// Attribute crate_type
//
// ferrocene-annotations: fls_9gprp17h6t1q
// Use Imports
