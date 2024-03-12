//@ run-fail
//@ compile-flags: -Cdebug-assertions=yes

fn main() {
    debug_assert!(false);
}

// ferrocene-annotations: um_rustc_C_debug_assertions
