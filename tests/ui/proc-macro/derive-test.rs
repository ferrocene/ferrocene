//@ run-pass
//@ no-prefer-dynamic
//@ compile-flags: --test

#![crate_type = "proc-macro"]

extern crate proc_macro;

use proc_macro::TokenStream;

// ```
// assert!(true);
// ```
#[proc_macro_derive(Foo)]
pub fn derive_foo(_input: TokenStream) -> TokenStream {
    "".parse().unwrap()
}

#[test]
pub fn test_derive() {
    assert!(true);
}

// ferrocene-annotations: fls_dv2j1fvvnk1t
// Attribute test
//
// ferrocene-annotations: fls_q6qecp6e413
// Attribute proc_macro_derive
//
// ferrocene-annotations: um_rustc_test
