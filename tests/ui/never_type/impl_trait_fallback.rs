//@ check-pass

fn main() {}

trait T {}
impl T for () {}

fn should_ret_unit() -> impl T {
    panic!()
}

// ferrocene-annotations: fls_lv7w7aalpwm5
// Type Inference
