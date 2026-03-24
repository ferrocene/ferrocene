//@ run-pass
//@ only-aarch64
//@ compile-flags: -Ctarget-cpu=cortex-a53

extern crate core;

#[target_feature(enable = "crc")]
// https://developer.arm.com/architectures/instruction-sets/intrinsics/__crc32b
// https://doc.rust-lang.org/nightly/core/arch/aarch64/index.html#functions
pub fn feat_crc32() {
    use core::arch::aarch64;

    assert_eq!(0x5501e732, aarch64::__crc32b(0x04C11DB7, 0x0_u8));
    assert_eq!(0xc8825067, aarch64::__crc32h(0x04C11DB7, 0x0_u16));
    assert_eq!(0, aarch64::__crc32w(0x04C11DB7, 0x04C11DB7_u32));
    assert_eq!(0, aarch64::__crc32d(0x04C11DB7, 0x04C11DB7_u64));

    assert_eq!(0xb30e42d0, aarch64::__crc32cb(0x1EDC6F41, 0x0_u8));
    assert_eq!(0xd360efe9, aarch64::__crc32ch(0x1EDC6F41, 0x0_u16));
    assert_eq!(0, aarch64::__crc32cw(0x1EDC6F41, 0x1EDC6F41_u32));
    assert_eq!(0, aarch64::__crc32cd(0x1EDC6F41, 0x1EDC6F41_u64));
}

fn main() {
    unsafe { feat_crc32(); }
}
