//@ check-pass
//@ needs-asm-support
#![crate_type = "lib"]

use std::arch::naked_asm;

#[unsafe(naked)]
pub extern "C" fn naked(p: char) -> u128 {
    //~^ WARN uses type `char`
    //~| WARN uses type `u128`
    naked_asm!("")
}

// ferrocene-annotations: fls_sd6rumpeb355
// Attribute naked
