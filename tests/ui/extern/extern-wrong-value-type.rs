extern "C" fn f() {
}

fn is_fn<F>(_: F) where F: Fn() {}

fn main() {
    // extern functions are extern "C" fn
    let _x: extern "C" fn() = f; // OK
    is_fn(f);
    //~^ ERROR expected a `Fn<()>` closure, found `extern "C" fn() {f}`
}

// ferrocene-annotations: fls_qcb1n9c0e5hz
// Functions
// ferrocene-annotations: fls_xztr1kebz8bo
// Function Pointer Types
// ferrocene-annotations: fls_usgd0xlijoxv
// ABI
