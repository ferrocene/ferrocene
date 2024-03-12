// -A and --allow can be specified multiple times, repeating the same option is not considered an
// error.
//
//@ check-pass
//@ compile-flags: -A overflowing_literals --allow overflowing_literals

fn main() {
    let x: u8 = 1000;
}

// ferrocene-annotations: um_rustc_A
