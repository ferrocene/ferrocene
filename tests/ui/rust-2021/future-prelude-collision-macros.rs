//@ run-rustfix
//@ edition:2018
//@ check-pass
#![warn(rust_2021_prelude_collisions)]
#![allow(unreachable_code)]

macro_rules! foo {
    () => {{
        123;
        S
    }};
}

trait MyTry<T> {
    fn try_into(self, _: u8);
}

struct S;

impl MyTry<i32> for S {
    fn try_into(self, _: u8) {}
}

trait TryFromU8: Sized {
    fn try_from(_: u8);
}

impl TryFromU8 for u32 {
    fn try_from(_: u8) {}
}

macro_rules! bar {
    () => {
        u32
    };
}

fn main() {
    foo!().try_into(todo!());
    //~^ WARNING trait method `try_into` will become ambiguous in Rust 2021
    //~| WARNING this is accepted in the current edition
    <bar!()>::try_from(0);
    //~^ WARNING trait-associated function `try_from` will become ambiguous in Rust 2021
    //~| WARNING this is accepted in the current edition
}

// ferrocene-annotations: fls_xa7lp0zg1ol2
// Declarative Macros
//
// ferrocene-annotations: fls_85vx1qfa061i
// Traits
//
// ferrocene-annotations: fls_vhpwge5123cm
// Generic Parameters
//
// ferrocene-annotations: fls_fk2m2irwpeof
// Implementations
//
// ferrocene-annotations: fls_z7q8kbjwdc7g
// Method Call Expressions
//
// ferrocene-annotations: fls_vnvt40pa48n8
// Macro Invocation
//
// ferrocene-annotations: fls_wqazkzle0ix9
// Method Resolution
