//@ compile-flags: -O

#![crate_type = "lib"]

#[inline(never)]
pub fn floats() -> u32 {
    // CHECK: ret i32 0
    let x = if true { -0.0 } else { 1.0 };
    if x == 0.0 { 0 } else { 1 }
}
