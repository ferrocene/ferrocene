//@ needs-rustc-debug-assertions

struct NonGeneric {}

#[derive(Default)]
//~^ ERROR: struct takes 0 lifetime arguments but 1 lifetime argument was supplied
//~| ERROR: struct takes 0 generic arguments but 1 generic argument was supplied
struct NonGeneric<'a, const N: usize> {}
//~^ ERROR: struct takes 0 lifetime arguments but 1 lifetime argument was supplied
//~| ERROR: struct takes 0 generic arguments but 1 generic argument was supplied
//~| ERROR: lifetime parameter `'a` is never used
//~| ERROR: the name `NonGeneric` is defined multiple times

pub fn main() {}

// Ferrocene addition: the output here differs slightly from upstream because the mono visitor
// changes the order that items are seen. We emit all the same errors, so we just --bless the
// differences in .stderr away.
