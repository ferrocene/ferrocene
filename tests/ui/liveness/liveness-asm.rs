// Ensure inout asm! operands are marked as used by the liveness pass

//@ only-x86_64
//@ check-pass

#![allow(dead_code)]
#![warn(unused_assignments)]
#![warn(unused_variables)]

use std::arch::asm;

// Test the single inout case
unsafe fn f1(mut src: *const u8) {
    asm!("/*{0}*/", inout(reg) src); //~ WARN value assigned to `src` is never read
}

unsafe fn f2(mut src: *const u8) -> *const u8 {
    asm!("/*{0}*/", inout(reg) src);
    src
}

// Test the split inout case
unsafe fn f3(mut src: *const u8) {
    asm!("/*{0}*/", inout(reg) src => src); //~ WARN value assigned to `src` is never read
}

unsafe fn f4(mut src: *const u8) -> *const u8 {
    asm!("/*{0}*/", inout(reg) src => src);
    src
}

// Tests the use of field projections
struct S {
    field: *mut u8,
}

unsafe fn f5(src: &mut S) {
    asm!("/*{0}*/", inout(reg) src.field);
}

unsafe fn f6(src: &mut S) {
    asm!("/*{0}*/", inout(reg) src.field => src.field);
}

fn main() {}

// ferrocene-annotations: fls_j9l8wn6wgm06
// Registers
//
// ferrocene-annotations: fls_pz2ioqchjtym
// Register Classes
//
// ferrocene-annotations: fls_hejgghwzblf
// Register Arguments
//
// ferrocene-annotations: fls_e0896uk0mdyl
// Assembly Instructions
//
// ferrocene-annotations: fls_lv19xysy1f7e
// Register Parameter Modifiers
//
// ferrocene-annotations: fls_6momhvgx4w21
// Directive Support
//
// ferrocene-annotations: fls_a3joqzqp1v9d
// ABI Clobbers
//
// ferrocene-annotations: fls_ylli0ortyegk
// Assembly Options
//
// ferrocene-annotations: fls_qezwyridmjob
// Macros asm and global_asm
