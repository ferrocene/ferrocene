//@ revisions: a

// Counterpart to `expected-error-wrong-rev.rs`

#[cfg(a)]
fn foo() {
    let x: u32 = 22_usize; //[a]~ ERROR mismatched types
}

fn main() { }

// ferrocene-annotations: fls_lv7w7aalpwm5
// Type Inference
//
// ferrocene-annotations: fls_exe4zodlwfez
// Type Unification
