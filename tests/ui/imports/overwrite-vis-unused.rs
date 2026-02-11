// Regression test for issues #152004 and #151124.

#![deny(unused)]

mod m {
    pub struct S {}
}

use m::*;
pub use m::*; //~ ERROR unused import: `m::*`

fn main() {}
