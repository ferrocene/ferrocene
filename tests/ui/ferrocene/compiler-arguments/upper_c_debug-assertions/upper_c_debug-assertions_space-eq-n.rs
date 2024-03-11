//@ run-pass
//@ compile-flags: -C debug-assertions=n

fn main() {
    debug_assert!(false);
}

// ferrocene-annotations: um_rustc_C_debug_assertions
