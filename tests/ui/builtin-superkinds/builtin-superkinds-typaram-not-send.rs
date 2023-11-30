// Basic test for traits inheriting from the builtin kinds.

trait Foo : Send { }

impl <T: Sync+'static> Foo for T { }
//~^ ERROR `T` cannot be sent between threads safely

fn main() { }

// ferrocene-annotations: fls_fk2m2irwpeof
// Implementations
//
// ferrocene-annotations: fls_jeoas4n6su4
// Trait and Lifetime Bounds
//
// ferrocene-annotations: fls_eiw4by8z75di
// Send and Sync
