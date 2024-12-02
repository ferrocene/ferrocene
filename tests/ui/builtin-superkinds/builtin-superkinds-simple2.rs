//@ check-pass
// Simple test case of implementing a trait with super-builtin-kinds.


trait Foo : Send { }

impl Foo for isize { }

pub fn main() { }

// ferrocene-annotations: fls_fk2m2irwpeof
// Implementations
//
// ferrocene-annotations: fls_jeoas4n6su4
// Trait and Lifetime Bounds
//
// ferrocene-annotations: fls_eiw4by8z75di
// Send and Sync
