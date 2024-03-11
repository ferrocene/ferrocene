//@ run-pass
#![allow(dead_code)]
//@ pretty-expanded FIXME #23616

#[derive(Clone)]
enum E<T,U> {
    A(T),
    B(T,U),
    C
}

pub fn main() {
    let _ = E::A::<isize, isize>(1).clone();
}

// ferrocene-annotations: fls_szibmtfv117b
// Enum Type
//
// ferrocene-annotations: fls_vhpwge5123cm
// Generic Parameters
