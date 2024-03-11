//@ When both -A and -D are specified for the same lint, the last one overrides
//@ the previous. So here, -D overrides -A, denying the lint.
//
//@ check-fail
//@ compile-flags: -A overflowing_literals -D overflowing_literals

fn main() {
    let x: u8 = 1000; //~ ERROR
}

// ferrocene-annotations: um_rustc_A
