extern "C" {
    fn foo() -> i32 { //~ ERROR incorrect function inside `extern` block
        return 0;
    }
}

extern "C" fn bar() -> i32 {
    return 0;
}

fn main() {}

// ferrocene-annotations: fls_yztwtek0y34v
// External Functions
// ferrocene-annotations: fls_qcb1n9c0e5hz
// Functions
// ferrocene-annotations: fls_usgd0xlijoxv
// ABI
