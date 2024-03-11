//@ run-pass
//@ compile-flags: -Cdebug-assertions=n

fn main() {
    debug_assert!(false);
}

// ferrocene-annotations: um_rustc_C_debug_assertions
