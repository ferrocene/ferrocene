// issue: rust-lang/rust#47446
//@ run-rustfix
//@ check-pass

#![warn(unused_attributes)]
#[no_mangle]
//~^ WARN `#[no_mangle]` attribute may not be used in combination with `#[export_name]` [unused_attributes]
#[export_name = "foo"]
pub fn bar() {}

#[unsafe(no_mangle)]
//~^ WARN `#[no_mangle]` attribute may not be used in combination with `#[export_name]` [unused_attributes]
#[export_name = "baz"]
pub fn bak() {}

fn main() {}

// ferrocene-annotations: fls_mvd7nz8k3wcy
// Attribute no_mangle
