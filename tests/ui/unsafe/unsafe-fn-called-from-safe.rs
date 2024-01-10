unsafe fn f() {
    return;
}

fn main() {
    f();
    //~^ ERROR call to unsafe function `f` is unsafe
}

// ferrocene-annotations: fls_qcb1n9c0e5hz
// Functions
//
// ferrocene-annotations: fls_xa4nbfas01cj
// Call Expressions
