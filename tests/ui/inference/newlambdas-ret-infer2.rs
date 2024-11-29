//@ run-pass

#![allow(dead_code)]
// Test that the lambda kind is inferred correctly as a return
// expression


fn unique() -> Box<dyn FnMut()+'static> { Box::new(|| ()) }

pub fn main() {
}

// ferrocene-annotations: fls_tjyexqrx0fx5
// Closure Expressions
//
// ferrocene-annotations: fls_8l74abhlxzdl
// Return Expressions
//
// ferrocene-annotations: fls_zjoamsr3dbqk
// Diverging Expressions
