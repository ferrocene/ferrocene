//@ run-pass

#![allow(dead_code)]
// Issue #521


fn f() {
    let _x = match true {
        true => { 10 }
        false => { return }
    };
}

pub fn main() { }

// ferrocene-annotations: fls_dw33yt5g6m0k
// Type Coercion
