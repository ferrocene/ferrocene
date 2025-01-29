//@ run-fail
//@ error-pattern:thread 'main' panicked
//@ error-pattern:attempt to multiply with overflow
//@ needs-subprocess
//@ compile-flags: -C debug-assertions

fn main() {
    let _x = 2i32.pow(1024);
}

// ferrocene-annotations: um_rustc_C_debug_assertions
