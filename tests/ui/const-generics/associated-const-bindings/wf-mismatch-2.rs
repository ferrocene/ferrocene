//@ known-bug: unknown
//@ check-pass

#![feature(min_generic_const_args)]
#![expect(incomplete_features)]

trait Trait { #[type_const] const CT: bool; }

fn f<const N: i32>() {
    let _: dyn Trait<CT = { N }>; // FIXME: this should yield a type mismatch (`bool` v `i32`)
}
fn main() {}
