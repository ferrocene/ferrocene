//@ needs-asm-support
//@ run-pass

use std::arch::global_asm;

global_asm!("");

fn main() {}

// ferrocene-annotations: fls_qezwyridmjob
// Macros: asm, global_asm, and naked_asm
