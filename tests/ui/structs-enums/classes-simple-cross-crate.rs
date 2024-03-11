//@ run-pass
//@ aux-build:cci_class.rs

extern crate cci_class;
use cci_class::kitties::cat;

pub fn main() {
  let nyan : cat = cat(52, 99);
  let kitty = cat(1000, 2);
  assert_eq!(nyan.how_hungry, 99);
  assert_eq!(kitty.how_hungry, 2);
}

// ferrocene-annotations: fls_9ucqbbd0s2yo
// Struct Types
//
// ferrocene-annotations: fls_maw4u1o8q37u
// Crates
