//@ edition:2015
#[derive(::Absolute)] //~ ERROR cannot find
                      //~| ERROR cannot find
struct S;

fn main() {}

// ferrocene-annotations: fls_r6gj1p4gajnq
// Attribute derive
