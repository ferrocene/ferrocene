//@ build-pass (FIXME(62277): could be check-pass?)
//@ force-host
//@ no-prefer-dynamic

#![crate_type = "proc-macro"]
#![deny(rust_2018_compatibility)]

extern crate proc_macro;

use proc_macro::TokenStream;

#[proc_macro_derive(Template, attributes(template))]
pub fn derive_template(input: TokenStream) -> TokenStream {
    input
}

// ferrocene-annotations: fls_ujig607lmwbm
// Attribute crate_type
//
// ferrocene-annotations: fls_gklst7joeo33
// External Crates
//
// ferrocene-annotations: fls_q6qecp6e413
// Attribute proc_macro_derive
//
// ferrocene-annotations: fls_o8s3r7m90q59
// Derive Macros
