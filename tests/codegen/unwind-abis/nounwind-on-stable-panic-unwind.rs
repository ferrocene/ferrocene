<<<PULL-UPSTREAM>>> file deleted upstream; fix the conflict and remove this line
//@ compile-flags: -C opt-level=0
//@ needs-unwind

#![crate_type = "lib"]

// We disable optimizations to prevent LLVM from inferring the attribute.

extern "C" {
    fn bar();
}

// CHECK-NOT: Function Attrs:{{.*}}nounwind
pub unsafe extern "C" fn foo() {
    bar();
}

// Note that this test will get removed when `C-unwind` is fully stabilized

// ferrocene-annotations: um_rustc_C_opt_level
