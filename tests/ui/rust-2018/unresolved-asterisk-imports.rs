//@ edition:2018

use not_existing_crate::*; //~ ERROR unresolved import `not_existing_crate
use std as foo;

fn main() {}

// ferrocene-annotations: fls_9gprp17h6t1q
// Use Imports
