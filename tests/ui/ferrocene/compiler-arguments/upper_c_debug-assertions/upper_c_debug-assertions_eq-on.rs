//@ run-fail
//@ compile-flags: -Cdebug-assertions=on

fn main() {
    debug_assert!(false);
}

// ferrocene-annotations: um_rustc_C_debug_assertions
