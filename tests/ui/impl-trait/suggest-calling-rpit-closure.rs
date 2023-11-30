fn whatever() -> i32 {
    opaque()
//~^ ERROR mismatched types
}

fn opaque() -> impl Fn() -> i32 {
    || 0
}

fn main() {
    let _ = whatever();
}

// ferrocene-annotations: fls_lv7w7aalpwm5
// Type Inference
//
// ferrocene-annotations: fls_exe4zodlwfez
// Type Unification
