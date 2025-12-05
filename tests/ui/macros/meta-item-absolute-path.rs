//@ edition:2015
#[derive(::Absolute)] //~ ERROR failed to resolve
                      //~| ERROR failed to resolve
struct S;

fn main() {}

// ferrocene-annotations: fls_r6gj1p4gajnq
// Attribute derive
