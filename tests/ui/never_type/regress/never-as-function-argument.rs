// Regression test for <https://github.com/rust-lang/rust/issues/13352>,
// check that the never type can be used as a function argument.
//
//@run-pass

fn foo(_: Box<dyn FnMut()>) {}

fn main() {
    #[expect(unreachable_code)]
    foo(loop {
        std::process::exit(0);
    });
}

// ferrocene-annotations: fls_lv7w7aalpwm5
// Type Inference
// ferrocene-annotations: fls_dw33yt5g6m0k
// Type Coercion
