//@ check-pass
//@ compile-flags: --crate-type=proc-macro
//@ needs-dynamic-linking - proc macros rely on dynamic linking

extern crate proc_macro;

use proc_macro::TokenStream;

#[proc_macro_derive(Template, attributes(template))]
pub fn foo(input: TokenStream) -> TokenStream {
    input
}

// ferrocene-annotations: um_rustc_crate_type
