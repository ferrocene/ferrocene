<<<PULL-UPSTREAM>>> file deleted upstream; move the Ferrocene annotations if any, and delete this file
//@ run-pass
#[derive(PartialEq, Debug)]
struct Foo;

pub fn main() {
  assert_eq!(Foo, Foo);
  assert!(!(Foo != Foo));
}

// ferrocene-annotations: fls_9ucqbbd0s2yo
// Struct Type

// ferrocene-annotations: fls_nsvzzbldhq53
// Comparison Expressions
