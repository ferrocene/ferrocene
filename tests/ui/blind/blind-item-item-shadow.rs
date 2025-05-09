//@ dont-require-annotations: NOTE

mod foo { pub mod foo {  } }

use foo::foo;
//~^ ERROR the name `foo` is defined multiple times
//~| NOTE `foo` reimported here

fn main() {}

// ferrocene-annotations: fls_9gprp17h6t1q
// Use Imports
//
// ferrocene-annotations: fls_ydmnb7qnmzzq
// Shadowing
//
// ferrocene-annotations: fls_exe4zodlwfez
// Type Unification
