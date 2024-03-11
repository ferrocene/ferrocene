//@ run-pass
//@ aux-build:cci_class_cast.rs

extern crate cci_class_cast;

use cci_class_cast::kitty::cat;

fn print_out(thing: Box<dyn ToString>, expected: String) {
  let actual = (*thing).to_string();
  println!("{}", actual);
  assert_eq!(actual.to_string(), expected);
}

pub fn main() {
  let nyan: Box<dyn ToString> = Box::new(cat(0, 2, "nyan".to_string())) as Box<dyn ToString>;
  print_out(nyan, "nyan".to_string());
}

// ferrocene-annotations: fls_1qhsun1vyarz
// Type Cast Expressions
//
// ferrocene-annotations: fls_qa98qdi42orq
// Trait Object Types
//
// ferrocene-annotations: fls_maw4u1o8q37u
// Crates
//
// ferrocene-annotations: fls_9ucqbbd0s2yo
// Struct Types
