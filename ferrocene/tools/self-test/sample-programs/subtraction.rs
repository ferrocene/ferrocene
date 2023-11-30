// SPDX-License-Identifier: MIT OR Apache-2.0
// SPDX-FileCopyrightText: The Ferrocene Developers

#![cfg_attr(selftest_no_std, no_std)]

#[no_mangle]
pub extern "C" fn subtraction_sub(left: i32, right: i32) -> i32 {
    left - right
}

#[cfg(selftest_no_std)]
#[panic_handler]
fn panic_handler(_: &core::panic::PanicInfo) -> ! {
    loop {}
}
