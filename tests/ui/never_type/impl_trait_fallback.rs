//@ check-pass

fn main() {}

trait T {}
impl T for () {}

fn should_ret_unit() -> impl T {
    //~^ warn: this function depends on never type fallback being `()`
    //~| warn: this was previously accepted by the compiler but is being phased out; it will become a hard error in Rust 2024 and in a future release in all editions!
    panic!()
}

// ferrocene-annotations: fls_lv7w7aalpwm5
// Type Inference
