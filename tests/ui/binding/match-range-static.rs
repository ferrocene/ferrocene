//@ run-pass
#![allow(non_upper_case_globals)]

const s: isize = 1;
const e: isize = 42;

pub fn main() {
    match 7 {
        s..=e => (),
        _ => (),
    }
}

// ferrocene-annotations: fls_fyskeih6twyb
// Range pattern matching
//
// ferrocene-annotations: fls_6tl1fx99yn6c
// Range patterns
