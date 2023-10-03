// revisions: old next
//[next] compile-flags: -Ztrait-solver=next

struct Foo<const N: u8 = { 255 + 1 }>;
//~^ ERROR evaluation of constant value failed
fn main() {}

// ferrocene-annotations: fls_66m4rnbssgig
// Constant Expressions
