// Test how overloaded deref interacts with borrows when DerefMut
// is implemented.

use std::ops::{Deref, DerefMut};

struct Own<T> {
    value: *mut T
}

impl<T> Deref for Own<T> {
    type Target = T;

    fn deref<'a>(&'a self) -> &'a T {
        unsafe { &*self.value }
    }
}

impl<T> DerefMut for Own<T> {
    fn deref_mut<'a>(&'a mut self) -> &'a mut T {
        unsafe { &mut *self.value }
    }
}

fn deref_imm(x: Own<isize>) {
    let __isize = &*x;
}

fn deref_mut1(x: Own<isize>) {
    let __isize = &mut *x; //~ ERROR cannot borrow
}

fn deref_mut2(mut x: Own<isize>) {
    let __isize = &mut *x;
}

fn deref_extend<'a>(x: &'a Own<isize>) -> &'a isize {
    &**x
}

fn deref_extend_mut1<'a>(x: &'a Own<isize>) -> &'a mut isize {
    &mut **x //~ ERROR cannot borrow
}

fn deref_extend_mut2<'a>(x: &'a mut Own<isize>) -> &'a mut isize {
    &mut **x
}

fn assign1<'a>(x: Own<isize>) {
    *x = 3; //~ ERROR cannot borrow
}

fn assign2<'a>(x: &'a Own<isize>) {
    **x = 3; //~ ERROR cannot borrow
}

fn assign3<'a>(x: &'a mut Own<isize>) {
    **x = 3;
}

pub fn main() {}

// ferrocene-annotations: fls_9ucqbbd0s2yo
// Struct Types
//
// ferrocene-annotations: fls_vhpwge5123cm
// Generic Parameters
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
