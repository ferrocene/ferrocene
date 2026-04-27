<<<PULL-UPSTREAM>>> file deleted upstream; move the Ferrocene annotations if any, and delete this file
//@ run-pass
#![allow(dead_code)]

#[derive(Clone)]
enum E {
    A,
    B(()),
    C
}

pub fn main() {
    let _ = E::A.clone();
}

// ferrocene-annotations: fls_szibmtfv117b
// Enum Type
