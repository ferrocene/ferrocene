//@ run-pass
pub struct Z(#[allow(dead_code)] &'static Z);

pub fn main() {}

// ferrocene-annotations: fls_9ucqbbd0s2yo
// Struct Types
