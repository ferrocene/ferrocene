//@ check-pass
//@ compile-flags: -Zinput-stats
//@ only-64bit
// layout randomization affects the hir stat output
//@ needs-deterministic-layouts

// Type layouts sometimes change. When that happens, until the next bootstrap
// bump occurs, stage1 and stage2 will give different outputs for this test.
// Add an `ignore-stage1` comment marker to work around that problem during
// that time.


// The aim here is to include at least one of every different type of top-level
// AST/HIR node reported by `-Zinput-stats`.

#![allow(dead_code)]

use std::arch::asm;
use std::fmt::Debug;
use std::ffi::c_void;

extern "C" { fn f(p: *mut c_void); }

/// An enum.
enum E<'a, T: Copy> { A { t: T }, B(&'a u32) }

trait Go {
    type G: Debug;
    fn go(self) -> u32;
}

impl<'a, T: Copy> Go for E<'a, T> {
    type G = bool;
    fn go(self) -> u32 {
        99
    }
}

fn f2<T>(t: T) where T: Debug {}

fn main() {
    let x = E::A { t: 3 };
    match x {
        E::A { .. } => {}
        _ => {}
    }

    unsafe { asm!("mov rdi, 1"); }
}

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
