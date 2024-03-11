//@ build-pass (FIXME(62277): could be check-pass?)
//@ compile-flags:--cfg my_feature

#![no_std]

#[cfg(my_feature)]
extern crate std;

mod m {
    #[cfg(my_feature)]
    fn conditional() {
        std::vec::Vec::<u8>::new(); // OK
    }
}

fn main() {}

// ferrocene-annotations: fls_gklst7joeo33
// External Crates
//
// ferrocene-annotations: um_rustc_cfg
