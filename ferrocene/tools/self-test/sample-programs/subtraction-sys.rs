// SPDX-License-Identifier: MIT OR Apache-2.0
// SPDX-FileCopyrightText: The Ferrocene Developers

#![cfg_attr(selftest_no_std, no_std)]

extern "C" {
    fn subtraction_sub(left: i32, right: i32) -> i32;
}

pub fn sub(left: i32, right: i32) -> i32 {
    unsafe { subtraction_sub(left, right) }
}
