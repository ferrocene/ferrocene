//@ build-fail
//@ compile-flags: -C debug-assertions

#![deny(arithmetic_overflow)]

fn main() {
    let _x = -1_i32 >> 32;
    //~^ ERROR: this arithmetic operation will overflow
}

// ferrocene-annotations: um_rustc_C_debug_assertions
