#![crate_type = "lib"]
#![allow(missing_abi)]

extern fn none_fn(x: bool) -> i32;
//~^ ERROR free function without a body
extern "C" fn c_fn(x: bool) -> i32;
//~^ ERROR free function without a body

// ferrocene-annotations: fls_qcb1n9c0e5hz
// Functions
// ferrocene-annotations: fls_usgd0xlijoxv
// ABI
