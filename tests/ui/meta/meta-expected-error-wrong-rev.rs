//@ ignore-compare-mode-polonius

//@ revisions: a
//@ should-fail

// This is a "meta-test" of the compilertest framework itself.  In
// particular, it includes the right error message, but the message
// targets the wrong revision, so we expect the execution to fail.
// See also `meta-expected-error-correct-rev.rs`.

#[cfg(a)]
fn foo() {
    let x: u32 = 22_usize; //[b]~ ERROR mismatched types
}

fn main() { }

// ferrocene-annotations: fls_lv7w7aalpwm5
// Type Inference
//
// ferrocene-annotations: fls_exe4zodlwfez
// Type Unification
