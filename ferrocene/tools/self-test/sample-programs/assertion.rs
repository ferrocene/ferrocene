// SPDX-License-Identifier: MIT OR Apache-2.0
// SPDX-FileCopyrightText: The Ferrocene Developers

#![cfg_attr(selftest_no_std, no_std)]
#![cfg_attr(selftest_no_std, no_main)]

use addition::add;
use subtraction_sys::sub;

pub fn main() {
    assert_eq!(2, add(sub(2, 1), 1));
}

#[cfg(selftest_no_std)]
#[panic_handler]
fn panic_handler(_: &core::panic::PanicInfo) -> ! {
    loop {}
}
