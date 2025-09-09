//@ needs-asm-support

use std::arch::naked_asm;

#[track_caller] //~ ERROR [E0736]
//~^ ERROR `#[track_caller]` requires Rust ABI
#[unsafe(naked)]
extern "C" fn f() {
    unsafe {
        naked_asm!("");
    }
}

struct S;

impl S {
    #[track_caller] //~ ERROR [E0736]
    //~^ ERROR `#[track_caller]` requires Rust ABI
    #[unsafe(naked)]
    extern "C" fn g() {
        unsafe {
            naked_asm!("");
        }
    }
}

fn main() {}

// ferrocene-annotations: fls_qcb1n9c0e5hz
// Functions

// ferrocene-annotations: fls_sd6rumpeb355
// Attribute naked
