//@ run-pass
//@ needs-asm-support

#![feature(naked_functions)]
#![allow(dead_code)]

#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
core::arch::global_asm!(
    r#"
    .global foo
    .global _foo
foo:
_foo:
    ret
"#
);

extern "C" {
    fn foo();
}

#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
fn main() {
    unsafe {
        foo();
    }
}

#[cfg(not(any(target_arch = "x86_64", target_arch = "x86")))]
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
