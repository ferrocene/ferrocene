//@ needs-asm-support

use std::arch::asm;

const _: () = unsafe { asm!("nop") };
//~^ ERROR inline assembly

fn main() {}

// ferrocene-annotations: fls_qezwyridmjob
// Macros asm and global_asm
