//@ run-pass




// Issue #50.

struct X { foo: String, bar: String }

pub fn main() {
    let x = X {foo: "hello".to_string(), bar: "world".to_string()};
    println!("{}", x.foo.clone());
    println!("{}", x.bar.clone());
}

// ferrocene-annotations: fls_8tsynkj2cufj
// Struct Expressions
//
// ferrocene-annotations: fls_lv7w7aalpwm5
// Type Inference
