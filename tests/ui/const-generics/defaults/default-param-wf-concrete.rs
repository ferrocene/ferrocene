//@ revisions: old next
//@[next] compile-flags: -Znext-solver

struct Foo<const N: u8 = { 255 + 1 }>;
//~^ ERROR overflow
fn main() {}

// ferrocene-annotations: fls_66m4rnbssgig
// Constant Expressions
