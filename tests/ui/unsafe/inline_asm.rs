//@ needs-asm-support

use std::arch::asm;

fn main() {
    asm!("nop"); //~ ERROR use of inline assembly is unsafe and requires unsafe function or block
}

// ferrocene-annotations: fls_qezwyridmjob
// Macros asm and global_asm
