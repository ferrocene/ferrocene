//@ run-pass
//@ aux-build:cci_class_6.rs

extern crate cci_class_6;
use cci_class_6::kitties::cat;

pub fn main() {
  let mut nyan : cat<char> = cat::<char>(52_usize, 99, vec!['p']);
  let mut kitty = cat(1000_usize, 2, vec!["tabby".to_string()]);
  assert_eq!(nyan.how_hungry, 99);
  assert_eq!(kitty.how_hungry, 2);
  nyan.speak(vec![1_usize,2_usize,3_usize]);
  assert_eq!(nyan.meow_count(), 55_usize);
  kitty.speak(vec!["meow".to_string(), "mew".to_string(), "purr".to_string(), "chirp".to_string()]);
  assert_eq!(kitty.meow_count(), 1004_usize);
}

// ferrocene-annotations: fls_9ucqbbd0s2yo
// Struct Types
//
// ferrocene-annotations: fls_maw4u1o8q37u
// Crates
//
// ferrocene-annotations: fls_fk2m2irwpeof
// Implementations
//
// ferrocene-annotations: fls_z7q8kbjwdc7g
// Method Call Expressions
//
// ferrocene-annotations: fls_18k3uajrgq5f
// Field Access Expressions
//
// ferrocene-annotations: fls_xcwfotmq2e5d
// Field Resolution
//
// ferrocene-annotations: fls_wqazkzle0ix9
// Method Resolution
