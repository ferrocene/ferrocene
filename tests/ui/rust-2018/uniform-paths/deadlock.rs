//@ edition:2018
//@ compile-flags:--extern foo --extern bar

use bar::foo; //~ ERROR can't find crate for `bar`
use foo::bar; //~ ERROR can't find crate for `foo`
//~^^ ERROR unresolved imports `bar::foo`, `foo::bar`

fn main() {}

// ferrocene-annotations: fls_9gprp17h6t1q
// Use Imports
