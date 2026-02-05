//@ known-bug: unknown
//@ check-pass

#![feature(min_generic_const_args)]
#![expect(incomplete_features)]

trait Trait { #[type_const] const CT: bool; }

trait Bound { #[type_const] const N: u32; }
impl Bound for () { #[type_const] const N: u32 = 0; }

fn f() { let _: dyn Trait<CT = { <() as Bound>::N }>; } // FIXME
fn g(_: impl Trait<CT = { <() as Bound>::N }>) {} // FIXME

fn main() {}
