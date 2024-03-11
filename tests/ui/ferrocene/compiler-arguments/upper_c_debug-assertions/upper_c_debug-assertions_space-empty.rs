//@ run-fail
//@ compile-flags: -C debug-assertions

fn main() {
    debug_assert!(false);
}

// ferrocene-annotations: um_rustc_C_debug_assertions
