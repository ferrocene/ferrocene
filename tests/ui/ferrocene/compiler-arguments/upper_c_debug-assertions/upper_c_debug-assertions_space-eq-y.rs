//@ run-fail
//@ compile-flags: -C debug-assertions=y

fn main() {
    debug_assert!(false);
}

// ferrocene-annotations: um_rustc_C_debug_assertions
