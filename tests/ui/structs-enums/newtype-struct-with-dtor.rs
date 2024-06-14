//@ run-pass
#![allow(unused_unsafe)]
#![allow(unused_variables)]
//@ pretty-expanded FIXME #23616

#[allow(dead_code)]
pub struct Fd(u32);

#[allow(dead_code)]
fn foo(a: u32) {}

impl Drop for Fd {
    fn drop(&mut self) {
        unsafe {
            let Fd(s) = *self;
            foo(s);
        }
    }
}

pub fn main() {
}

// ferrocene-annotations: fls_u2mzjgiwbkz0
// Destructors
//
// ferrocene-annotations: fls_9ucqbbd0s2yo
// Struct Types
