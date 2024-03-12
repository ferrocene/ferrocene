//@ run-pass

fn foo() -> isize { 22 }

pub fn main() {
    let mut x: Vec<extern "Rust" fn() -> isize> = Vec::new();
    x.push(foo);
    assert_eq!((x[0])(), 22);
}

// ferrocene-annotations: fls_hbbek3z4wtcs
// Function Types
//
// ferrocene-annotations: fls_vhpwge5123cm
// Generic Parameters
