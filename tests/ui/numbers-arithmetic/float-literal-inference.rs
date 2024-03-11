//@ run-pass
struct S {
    z: f64
}

pub fn main() {
    let x: f32 = 4.0;
    println!("{}", x);
    let y: f64 = 64.0;
    println!("{}", y);
    let z = S { z: 1.0 };
    println!("{}", z.z);
}

// ferrocene-annotations: fls_29tlg1vyqay2
// Float Literals
//
// ferrocene-annotations: fls_lv7w7aalpwm5
// Type Inference
//
// ferrocene-annotations: fls_h0dvogc64tfh
// Literal Expressions
//
// ferrocene-annotations: fls_e7zgqroy2qxn
// Value Expressions
