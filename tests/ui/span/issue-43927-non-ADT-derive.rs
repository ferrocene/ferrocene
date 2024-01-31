#![derive(Debug, PartialEq, Eq)] // should be an outer attribute!
//~^ ERROR `derive` attribute cannot be used at crate level
struct DerivedOn;

fn main() {}

// ferrocene-annotations: fls_r6gj1p4gajnq
// Attribute derive
