//@ assembly-output: emit-asm
//@ compile-flags: -Ctarget-cpu=cortex-a53
//@ only-aarch64

#![crate_type = "lib"]
#![no_std]

// CHECK-LABEL: feat_crc32:
#[no_mangle]
#[target_feature(enable = "crc")]
// https://developer.arm.com/architectures/instruction-sets/intrinsics/__crc32b
// https://doc.rust-lang.org/nightly/core/arch/aarch64/index.html#functions
pub fn feat_crc32() {
    use core::arch::aarch64;
    use core::hint::black_box;

    // CHECK: crc32b
    black_box(aarch64::__crc32b(0x04C11DB7, 0x0_u8));
    // CHECK: crc32h
    black_box(aarch64::__crc32h(0x04C11DB7, 0x0_u16));
    // CHECK: crc32w
    black_box(aarch64::__crc32w(0x04C11DB7, 0x04C11DB7_u32));
    // CHECK: crc32x
    black_box(aarch64::__crc32d(0x04C11DB7, 0x04C11DB7_u64));

    // CHECK: crc32cb
    black_box(aarch64::__crc32cb(0x1EDC6F41, 0x0_u8));
    // CHECK: crc32ch
    black_box(aarch64::__crc32ch(0x1EDC6F41, 0x0_u16));
    // CHECK: crc32cw
    black_box(aarch64::__crc32cw(0x1EDC6F41, 0x1EDC6F41_u32));
    // CHECK: crc32cx
    black_box(aarch64::__crc32cd(0x1EDC6F41, 0x1EDC6F41_u64));
}
