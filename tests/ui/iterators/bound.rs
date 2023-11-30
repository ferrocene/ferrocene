struct S<I: Iterator>(I);
struct T(S<u8>);
//~^ ERROR is not an iterator
fn main() {}

// ferrocene-annotations: fls_i7g2n7hfg3ch
// Generic Conformance
