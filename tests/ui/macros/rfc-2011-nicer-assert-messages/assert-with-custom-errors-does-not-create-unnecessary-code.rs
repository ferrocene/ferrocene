// ferrocene addition: support panic=abort targets; upstreamed in rust-lang/rust#135823
//@ compile-flags: --test -Zpanic_abort_tests
//@ run-pass

#![feature(core_intrinsics, generic_assert)]

#[should_panic(expected = "Custom user message")]
#[test]
fn test() {
  assert!(1 == 3, "Custom user message");
}

fn main() {
}

// ferrocene-annotations: um_rustc_test
