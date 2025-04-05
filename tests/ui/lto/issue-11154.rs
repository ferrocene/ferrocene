//@ build-fail
//@ compile-flags: -C lto -C prefer-dynamic

fn main() {}

<<<<<<< HEAD
// ferrocene-annotations: um_rustc_C_prefer_dynamic
=======
//~? ERROR cannot prefer dynamic linking when performing LTO
>>>>>>> pull-upstream-temp--do-not-use-for-real-code
