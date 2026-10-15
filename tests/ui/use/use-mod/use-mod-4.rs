//@ edition: 2015
use crate::foo::self; //~ ERROR unresolved import `crate::foo`

use std::mem::self;

fn main() {}
