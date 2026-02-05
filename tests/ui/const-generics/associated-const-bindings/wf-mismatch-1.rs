//@ known-bug: unknown
//@ check-pass

#![feature(min_generic_const_args)]
#![expect(incomplete_features)]

trait Trait { #[type_const] const CT: bool; }

// FIXME: this should yield a type mismatch (`bool` v `i32`)
fn f<const N: i32>(_: impl Trait<CT = { N }>) {}

fn main() {}
