// test.c and its static library checkrust.rs make use of variadic functions (VaList).
// This test checks that the use of this feature does not
// prevent the creation of a functional binary.
// See https://github.com/rust-lang/rust/pull/49878

//@ needs-target-std
//@ ignore-android: FIXME(#142855)
//@ ignore-sgx: (x86 machine code cannot be directly executed)

// Ferrocene addition: <assert.h> is not available for inclusion on these platforms
//@ ignore-thumbv7em-ferrocene.facade-eabi
//@ ignore-thumbv7em-ferrocene.facade-eabihf

// Ferrocene addition: This test fails with linker errors on `-lpthread`, `-ldl`, and `-lrt` on
// these platforms
//@ ignore-armv7r-ferrocene.facade-eabihf
//@ ignore-aarch64-unknown-nto-qnx710
//@ ignore-x86_64-pc-nto-qnx710
//@ ignore-aarch64-unknown-nto-qnx800
//@ ignore-x86_64-pc-nto-qnx800

// Ferrocene addition: libcheckrust includes symbols which cannot be used without `-fPIC` on
// these platforms
//@ ignore-aarch64r82-unknown-ferrocene.facade
//@ ignore-aarch64v8r-unknown-ferrocene.facade
//@ ignore-aarch64-unknown-ferrocene.facade

use run_make_support::{cc, extra_c_flags, run, rustc, static_lib_name};

fn main() {
    rustc().edition("2021").input("checkrust.rs").run();
    cc().input("test.c")
        .input(static_lib_name("checkrust"))
        .out_exe("test")
        .args(extra_c_flags())
        .run();
    run("test");
}
