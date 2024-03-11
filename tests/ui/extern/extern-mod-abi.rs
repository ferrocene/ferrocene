//@ run-pass
#![allow(dead_code)]
//@ pretty-expanded FIXME #23616

extern "C" {
    fn pow(x: f64, y: f64) -> f64;
}

pub fn main() {}

// ferrocene-annotations: fls_yztwtek0y34v
// External Functions
