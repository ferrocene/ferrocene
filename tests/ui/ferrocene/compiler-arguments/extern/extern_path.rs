//@ check-pass
//@ aux-build:some_crate.rs
//@ compile-flags: --extern some_crate={{build-base}}/ferrocene/compiler-arguments/extern/extern_path/auxiliary/libsome_crate.so
//@ edition:2018
//@ only-linux Because there's no substitution for the library path
//@ needs-dynamic-linking - this test is attempting to link a dynamic library

fn main() {
    ::some_crate::hello();
}

// ferrocene-annotations: um_rustc_extern
