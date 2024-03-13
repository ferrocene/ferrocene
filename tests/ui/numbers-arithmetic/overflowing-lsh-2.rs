<<<PULL-UPSTREAM>>> file deleted by Rust, fix the conflict and remove this line
//@ build-fail
//@ compile-flags: -C debug-assertions

#![deny(arithmetic_overflow)]

fn main() {
    let _x = 1 << -1;
    //~^ ERROR: this arithmetic operation will overflow
}

// ferrocene-annotations: um_rustc_C_debug_assertions
