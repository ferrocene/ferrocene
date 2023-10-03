#![allow(non_camel_case_types)]

struct hello(isize);

fn main() {
    let hello = 0; //~ERROR let bindings cannot shadow tuple structs
}

// ferrocene-annotations: fls_ydmnb7qnmzzq
// Shadowing
// ferrocene-annotations: fls_7bxv8lybxm18
// Identifier Patterns
