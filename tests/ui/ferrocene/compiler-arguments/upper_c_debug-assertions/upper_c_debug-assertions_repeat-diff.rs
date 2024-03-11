// When repeated, the last one overrides the previous
//
//@ run-pass
//@ compile-flags: -Cdebug-assertions=y -Cdebug-assertions=n

fn main() {
    debug_assert!(false);
}

// ferrocene-annotations: um_rustc_C_debug_assertions
