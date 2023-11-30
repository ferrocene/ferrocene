fn foo(_: Box<dyn FnMut()>) {}

fn main() {
    foo(loop {
        std::process::exit(0);
    });
    2_usize + (loop {});
    //~^ ERROR E0277
}

// ferrocene-annotations: fls_lv7w7aalpwm5
// Type Inference
// ferrocene-annotations: fls_dw33yt5g6m0k
// Type Coercion
