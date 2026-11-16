//@ needs-rustc-debug-assertions

struct NonGeneric {}

#[derive(Default)]
struct NonGeneric<'a, const N: usize> {}
//~^ ERROR: the name `NonGeneric` is defined multiple times

pub fn main() {}

// Ferrocene addition: the output here differs slightly from upstream because the mono visitor
// changes the order that items are seen. We emit all the same errors, so we just --bless the
// differences in .stderr away.
