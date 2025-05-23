//@ build-fail
//@ compile-flags:-C panic=abort -C prefer-dynamic
//@ needs-unwind
//@ needs-crate-type: dylib

// This is a test where the local crate, compiled with `panic=abort`, links to
// the standard library **dynamically** which is already linked against
// `panic=unwind`. We should fail because the linked panic runtime does not
// correspond with our `-C panic` option.
//
// Note that this test assumes that the dynamic version of the standard library
// is linked to `panic_unwind`, which is currently the case.

fn main() {
}

//~? ERROR the linked panic runtime `panic_unwind` is not compiled with this crate's panic strategy `abort`

// ferrocene-annotations: um_rustc_C_panic
