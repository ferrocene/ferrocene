<<<PULL-UPSTREAM>>> file deleted upstream; move the Ferrocene annotations if any, and delete this file
//@ check-pass
#![expect(dependency_on_unit_never_type_fallback)]

fn main() {}

trait T {}
impl T for () {}

fn should_ret_unit() -> impl T {
    panic!()
}

// ferrocene-annotations: fls_lv7w7aalpwm5
// Type Inference
