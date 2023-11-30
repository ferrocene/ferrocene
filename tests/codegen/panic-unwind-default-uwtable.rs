// compile-flags: -C panic=unwind -C no-prepopulate-passes

#![crate_type = "lib"]

// CHECK: attributes #{{.*}} uwtable
pub fn foo() {}

// ferrocene-annotations: um_rustc_C_panic
