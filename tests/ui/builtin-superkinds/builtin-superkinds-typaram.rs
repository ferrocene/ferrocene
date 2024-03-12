//@ check-pass
// Tests correct implementation of traits with super-builtin-kinds
// using a bounded type parameter.

//@ pretty-expanded FIXME #23616

trait Foo : Send { }

impl <T: Send> Foo for T { }

pub fn main() { }

// ferrocene-annotations: fls_fk2m2irwpeof
// Implementations
//
// ferrocene-annotations: fls_jeoas4n6su4
// Trait and Lifetime Bounds
//
// ferrocene-annotations: fls_eiw4by8z75di
// Send and Sync
