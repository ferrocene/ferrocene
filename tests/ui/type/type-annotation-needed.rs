fn foo<T: Into<String>>(x: i32) {}
//~^ NOTE required by
//~| NOTE required by

fn main() {
    foo(42);
    //~^ ERROR type annotations needed
    //~| NOTE cannot infer type
    //~| NOTE cannot satisfy
}

// ferrocene-annotations: fls_lv7w7aalpwm5
// Type Inference
// ferrocene-annotations: fls_vhpwge5123cm
// Generic Parameters
// ferrocene-annotations: fls_xa4nbfas01cj
// Call Expressions
