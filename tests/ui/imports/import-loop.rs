//@ edition: 2015
use y::x;

mod y {
    pub use y::x; //~ ERROR unresolved import `y::x`
}

fn main() { }
