//@ check-pass

use std::io::Write;

fn main() -> Result<(), std::io::Error> {
    vec! { 1, 2, 3 }.len();
    write! { vec![], "" }?;
    println!{""}
    [0]; // separate statement, not indexing into the result of println.
    Ok(())
}

// ferrocene-annotations: fls_vnvt40pa48n8
// Macro Invocation
