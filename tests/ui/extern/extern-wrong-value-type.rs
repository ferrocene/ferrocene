//@ run-pass

extern "C" fn f() {
}

fn is_fn<F>(f: F) where F: Fn() {
    f();
}

fn main() {
    let _x: extern "C" fn() = f; // OK
    is_fn(f);
}

// ferrocene-annotations: fls_qcb1n9c0e5hz
// Functions
// ferrocene-annotations: fls_xztr1kebz8bo
// Function Pointer Types
// ferrocene-annotations: fls_usgd0xlijoxv
// ABI
