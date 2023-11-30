pub use std::panic;

#[macro_export]
macro_rules! panic { () => {} } //~ ERROR the name `panic` is defined multiple times

fn main() {}

// ferrocene-annotations: fls_xa7lp0zg1ol2
// Declarative Macros
