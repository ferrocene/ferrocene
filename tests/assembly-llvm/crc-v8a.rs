//@ compile-flags: --target aarch64-unknown-none -Ctarget-cpu=cortex-a53
//@ needs-llvm-components: aarch64
//@ assembly-output: emit-asm

#![crate_type = "lib"]
#![no_std]

// CHECK-LABEL: feat_crc32:
#[no_mangle]
pub fn feat_crc32() {
    // instruction is present when FEAT_CRC32 is implemented
    // CHECK: crc32b
    unsafe { core::arch::asm!("crc32b w0, w1, w2") }
}
