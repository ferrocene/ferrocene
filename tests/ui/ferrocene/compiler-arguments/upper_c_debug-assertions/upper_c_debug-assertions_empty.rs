//@ run-fail
//@ compile-flags: -Cdebug-assertions

fn main() {
    debug_assert!(false);
}

// ferrocene-annotations: um_rustc_C_debug_assertions
