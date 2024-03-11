//@ run-pass
#![allow(dead_code)]

const fn add(x: usize, y: usize) -> usize {
    x + y
}

const ARR: [i32; add(1, 2)] = [5, 6, 7];

pub fn main() {}

// ferrocene-annotations: fls_qcb1n9c0e5hz
// Functions
