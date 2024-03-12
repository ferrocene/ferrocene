// When both -D and -A are specified for the same lint, the last one overrides
// the previous. So here, -A overrides the previous -D, allowing the lint.
//
//@ check-pass
//@ compile-flags: -D overflowing_literals -A overflowing_literals

fn main() {
    let x: u8 = 1000;
}

// ferrocene-annotations: um_rustc_A
