//@ run-pass
//@ compile-flags: -Cdebug-assertions=no

fn main() {
    debug_assert!(false);
}

// ferrocene-annotations: um_rustc_C_debug_assertions
