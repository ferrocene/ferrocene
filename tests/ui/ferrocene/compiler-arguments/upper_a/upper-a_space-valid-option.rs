// When -A is followed by a valid option, the lint is allowed
//
//@ check-pass
//@ compile-flags: -A overflowing_literals

fn main() {
    let x: u8 = 1000;
}

// ferrocene-annotations: um_rustc_A
