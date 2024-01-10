unsafe fn f() {
    return;
}

fn main() {
    let x = f;
    x();
    //~^ ERROR call to unsafe function `f` is unsafe
}

// ferrocene-annotations: fls_qcb1n9c0e5hz
// Functions
//
// ferrocene-annotations: fls_yivm43r5wnp1
// Let Statements
//
// ferrocene-annotations: fls_lv7w7aalpwm5
// Type Inference
//
// ferrocene-annotations: fls_xa4nbfas01cj
// Call Expressions
