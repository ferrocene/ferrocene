//@ build-fail

//
#![crate_type="rlib"]
#![allow(warnings)]

#[export_name="fail"]
pub fn a() {
}

#[export_name="fail"]
pub fn b() {
//~^ ERROR symbol `fail` is already defined
}

// ferrocene-annotations: fls_osd6c4utyjb3
// FFI
//
// ferrocene-annotations: fls_pgp7ezcc9lh8
// Foreign Function Interface Attributes
//
// ferrocene-annotations: fls_olzilmy8n0nl
// Attribute export_name
