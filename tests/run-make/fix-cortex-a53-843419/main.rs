#![no_std]
#![no_main]

use core::arch::{asm, naked_asm};
use core::panic::PanicInfo;
use core::sync::atomic::AtomicU64;

// this variable needs to be 8-byte aligned because of the LDR instruction used below
// `AtomicU64` ensures the proper alignment
static SOME_VALUE: AtomicU64 = AtomicU64::new(0);

#[no_mangle]
// some targets may codegen a prologue for the frame pointer setup; `#[naked]`
// ensures the instructions appear at the expected addresses regardless of
// differences in codegen settings
#[unsafe(naked)]
extern "C" fn _start() {
    // 1. An ADRP instruction, which writes to a register Rn.
    // 2. A load or store instruction which does not write to Rn
    // 3. <an optional additional instruction>
    // 4. A load or store (unsigned immediate) using Rn
    naked_asm!(
        "nop",
        "nop",
        "adrp x0, {global}",
        "ldr x1, [x1, #0]",
        "ldr x0, [x0, :lo12:{global}]",
        global = sym SOME_VALUE,
    );
}

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {
        unsafe {
            asm!("udf #0");
        }
    }
}
