trait Foo {}

struct Bar;

impl Foo + Owned for Bar {} //~ ERROR expected a trait, found type

fn main() { }

// ferrocene-annotations: fls_jeoas4n6su4
// Trait and Lifetime Bounds
