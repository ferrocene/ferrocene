#[issue_100199::struct_with_bound] //~ ERROR cannot find trait `MyTrait` in the crate root
struct Foo {}
// The above must be on the first line so that it's span points to pos 0.
// This used to trigger an ICE because the diagnostic emitter would get
// an unexpected dummy span (lo == 0 == hi) while attempting to print a
// suggestion.

//@ proc-macro: issue-100199.rs

extern crate issue_100199;

mod traits {
    pub trait MyTrait {}
}

fn main() {}

// ferrocene-annotations: fls_4vjbkm4ceymk
// Attribute Macros
//
// ferrocene-annotations: fls_qkmkev85o5jf
// Attribute proc_macro
//
// ferrocene-annotations: fls_ejhlylrcajo
// Attribute proc_macro_attribute
//
// ferrocene-annotations: fls_wn1i6hzg2ff7
// Procedural Macros
