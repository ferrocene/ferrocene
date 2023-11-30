#![crate_type = "lib"]

fn foo<T, U>(_: U) {}

fn bar() {
    foo(|| {}); //~ ERROR type annotations needed
}

// ferrocene-annotations: fls_lv7w7aalpwm5
// Type Inference
//
// ferrocene-annotations: fls_8gpcpvc99pxj
// Call Conformance
