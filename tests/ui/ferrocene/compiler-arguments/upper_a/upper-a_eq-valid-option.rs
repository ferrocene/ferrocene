// If a valid option is set for -A, but an equal symbol is used as a delimiter,
// we have a failure and a hint.
//
// check-fail
// compile-flags: -A=overflowing_literals

fn main() {
    let x: u8 = 1000; // No error is detected here
}

// ferrocene-annotations: um_rustc_A
