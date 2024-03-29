//@ check-pass
//@ edition:2018

#![allow(non_snake_case)]

use std::rc::Rc;

struct Struct<'a> { x: &'a u32 }

type Alias<'a> = Struct<'a>;

impl<'a> Alias<'a> {
    async fn take_self(self, f: &u32) -> &u32 {
        f
    }

    async fn take_Alias(self: Alias<'a>, f: &u32) -> &u32 {
        f
    }

    async fn take_Box_Alias(self: Box<Alias<'a>>, f: &u32) -> &u32 {
        f
    }

    async fn take_Box_Box_Alias(self: Box<Box<Alias<'a>>>, f: &u32) -> &u32 {
        f
    }

    async fn take_Rc_Alias(self: Rc<Alias<'a>>, f: &u32) -> &u32 {
        f
    }

    async fn take_Box_Rc_Alias(self: Box<Rc<Alias<'a>>>, f: &u32) -> &u32 {
        f
    }
}

fn main() { }

// ferrocene-annotations: fls_vhpwge5123cm
// Generic Parameters
//
// ferrocene-annotations: fls_yqcygq3y6m5j
// Lifetimes
//
// ferrocene-annotations: fls_fk2m2irwpeof
// Implementations
