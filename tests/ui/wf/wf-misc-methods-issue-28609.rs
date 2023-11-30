// check that misc. method calls are well-formed

use std::marker::PhantomData;
use std::ops::{Deref, Shl};

#[derive(Copy, Clone)]
struct S<'a, 'b: 'a> {
    marker: PhantomData<&'a &'b ()>,
    bomb: Option<&'b u32>
}

type S2<'a> = S<'a, 'a>;

impl<'a, 'b> S<'a, 'b> {
    fn transmute_inherent(&self, a: &'b u32) -> &'a u32 {
        a
    }
}

fn return_dangling_pointer_inherent(s: S2) -> &u32 {
    let s = s;
    s.transmute_inherent(&mut 42) //~ ERROR cannot return value referencing temporary value
}

impl<'a, 'b> Deref for S<'a, 'b> {
    type Target = &'a u32;
    fn deref(&self) -> &&'a u32 {
        self.bomb.as_ref().unwrap()
    }
}

fn return_dangling_pointer_coerce(s: S2) -> &u32 {
    let four = 4;
    let mut s = s;
    s.bomb = Some(&four);
    &s //~ ERROR cannot return value referencing local variable `four`
}

fn return_dangling_pointer_unary_op(s: S2) -> &u32 {
    let four = 4;
    let mut s = s;
    s.bomb = Some(&four);
    &*s //~ ERROR cannot return value referencing local variable `four`
}

impl<'a, 'b> Shl<&'b u32> for S<'a, 'b> {
    type Output = &'a u32;
    fn shl(self, t: &'b u32) -> &'a u32 { t }
}

fn return_dangling_pointer_binary_op(s: S2) -> &u32 {
    let s = s;
    s << &mut 3 //~ ERROR cannot return value referencing temporary value
}

fn return_dangling_pointer_method(s: S2) -> &u32 {
    let s = s;
    s.shl(&mut 3) //~ ERROR cannot return value referencing temporary value
}

fn return_dangling_pointer_ufcs(s: S2) -> &u32 {
    let s = s;
    S2::shl(s, &mut 3) //~ ERROR cannot return value referencing temporary value
}

fn main() {
    let s = S { marker: PhantomData, bomb: None };
    let _inherent_dp = return_dangling_pointer_inherent(s);
    let _coerce_dp = return_dangling_pointer_coerce(s);
    let _unary_dp = return_dangling_pointer_unary_op(s);
    let _binary_dp = return_dangling_pointer_binary_op(s);
    let _method_dp = return_dangling_pointer_method(s);
    let _ufcs_dp = return_dangling_pointer_ufcs(s);
}

// ferrocene-annotations: fls_9ucqbbd0s2yo
// Struct Types
//
// ferrocene-annotations: fls_vhpwge5123cm
// Generic Parameters
//
// ferrocene-annotations: fls_jeoas4n6su4
// Trait and Lifetime Bounds
//
// ferrocene-annotations: fls_yqcygq3y6m5j
// Lifetimes
//
// ferrocene-annotations: fls_fk2m2irwpeof
// Implementations
//
// ferrocene-annotations: fls_l21tjqjkkaa0
// Associated Items
//
// ferrocene-annotations: fls_142vncdktbin
// Reference Types
//
// ferrocene-annotations: fls_qztk0bkju9u
// Borrow Expression
//
// ferrocene-annotations: fls_a14slch83hzn
// Borrowing
//
// ferrocene-annotations: fls_5cm4gkt55hjh
// Dereference Expression
