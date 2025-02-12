//! Check that `rustc`'s `--crate-type` flag accepts `--crate-type=<valid_type>` as well as the
//! multi-value version `--crate-type=<valid_type_1>,<valid_type_2>`.
//!
//! This test does not try to check if the output artifacts are valid.

// FIXME(#132309): add a proper `supports-crate-type` directive.

// Single valid crate types should pass
//@ revisions: lib rlib staticlib dylib cdylib bin proc_dash_macro

//@[lib] compile-flags: --crate-type=lib
//@[lib] check-pass

//@[rlib] compile-flags: --crate-type=rlib
//@[rlib] check-pass

//@[staticlib] compile-flags: --crate-type=staticlib
//@[staticlib] check-pass

<<<<<<< HEAD
//@[dylib] ignore-musl (dylibs are not supported)
//@[dylib] ignore-wasm (dylibs are not supported)
//@[dylib] ignore-ferrocenecoretest (dylibs are not supported)
//@[dylib] compile-flags: --crate-type=dylib
//@[dylib] check-pass

//@[cdylib] ignore-musl (cdylibs are not supported)
//@[cdylib] ignore-ferrocenecoretest (cdylibs are not supported)
=======
//@[dylib] ignore-musl (dylib is supported, but musl libc is statically linked by default)
//@[dylib] ignore-wasm (dylib is not supported)
//@[dylib] compile-flags: --crate-type=dylib
//@[dylib] check-pass

//@[cdylib] ignore-musl (cdylib is supported, but musl libc is statically linked by default)
>>>>>>> pull-upstream-temp--do-not-use-for-real-code
//@[cdylib] compile-flags: --crate-type=cdylib
//@[cdylib] check-pass

//@[bin] compile-flags: --crate-type=bin
//@[bin] check-pass

//@[proc_dash_macro] ignore-wasm (proc-macro is not supported)
//@[proc_dash_macro] ignore-ferrocenecoretest (proc-macro is not supported)
//@[proc_dash_macro] needs-unwind (panic=abort causes warning to be emitted)
//@[proc_dash_macro] compile-flags: --crate-type=proc-macro
//@[proc_dash_macro] check-pass

//@ revisions: multivalue multivalue_combined

//@[multivalue] compile-flags: --crate-type=lib,rlib,staticlib
//@[multivalue] check-pass

<<<<<<< HEAD
//@[multivalue_combined] ignore-musl (dylibs are not supported)
//@[multivalue_combined] ignore-wasm (dylibs are not supported)
//@[multivalue_combined] ignore-ferrocenecoretest (covered in `multivalue_combined_ferrocene`)
//@[multivalue_combined] compile-flags: --crate-type=lib,rlib,staticlib --crate-type=dylib
=======
//@[multivalue_combined] compile-flags: --crate-type=lib,rlib --crate-type=staticlib
>>>>>>> pull-upstream-temp--do-not-use-for-real-code
//@[multivalue_combined] check-pass

//@ revisions: multivalue_combined_ferrocene
//@[multivalue_combined_ferrocene] compile-flags: --crate-type=lib,rlib --crate-type=staticlib
//@[multivalue_combined_ferrocene] check-pass

// `proc-macro` is accepted, but `proc_macro` is not.
//@ revisions: proc_underscore_macro
//@[proc_underscore_macro] compile-flags: --crate-type=proc_macro
//@[proc_underscore_macro] error-pattern: "unknown crate type: `proc_macro`"

// Empty `--crate-type` not accepted.
//@ revisions: empty_crate_type
//@[empty_crate_type] compile-flags: --crate-type=
//@[empty_crate_type] error-pattern: "unknown crate type: ``"

// Random unknown crate type. Also check that we can handle non-ASCII.
//@ revisions: unknown
//@[unknown] compile-flags: --crate-type=🤡
//@[unknown] error-pattern: "unknown crate type: `🤡`"

fn main() {}

// ferrocene-annotations: um_rustc_crate_type
