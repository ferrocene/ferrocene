//@ check-fail
//@ compile-flags: --crate-type=proc-macro
//@ needs-dynamic-linking - proc macros rely on dynamic linking

extern crate proc_macro;

use proc_macro::TokenStream;

pub fn foo(input: TokenStream) -> TokenStream { //~ ERROR `proc-macro` crate
    input
}

// ferrocene-annotations: um_rustc_crate_type
