//@ run-fail
//@ compile-flags: -Cdebug-assertions=on -Copt-level=1

fn main() {
    debug_assert!(false);
}

// ferrocene-annotations: um_rustc_C_debug_assertions
