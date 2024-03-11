//@ run-fail
//@ compile-flags: -Cdebug-assertions=y

fn main() {
    debug_assert!(false);
}

// ferrocene-annotations: um_rustc_C_debug_assertions
