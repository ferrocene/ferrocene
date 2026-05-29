//@ compile-flags: -C no-prepopulate-passes -C panic=abort -C force-unwind-tables=n
//@ ignore-windows: unwind tables are required for panics on Windows

#![crate_type = "lib"]

// CHECK-LABEL: define{{.*}}void @foo
// CHECK-NOT: attributes #{{.*}} uwtable
#[no_mangle]
fn foo() {
    panic!();
}
<<<<<<< ferrocene/main

// ferrocene-annotations: um_rustc_C_panic
||||||| 62f36da19c6
=======

// CHECK-NOT: !"uwtable"
>>>>>>> rust-lang/rust/HEAD--generated-by-pull-upstream
