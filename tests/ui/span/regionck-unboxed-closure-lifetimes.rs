#![feature(rustc_attrs)]
use std::ops::FnMut;

fn main() { #![rustc_error] // rust-lang/rust#49855
    let mut f;
    {
        let c = 1;
        let c_ref = &c;
        //~^ ERROR `c` does not live long enough
        f = move |a: isize, b: isize| { a + b + *c_ref };
    }
    f.use_mut();
}

trait Fake { fn use_mut(&mut self) { } fn use_ref(&self) { }  }
impl<T> Fake for T { }

// ferrocene-annotations: fls_a14slch83hzn
// Borrowing
//
// ferrocene-annotations: fls_qztk0bkju9u
// Borrow Expression
//
// ferrocene-annotations: fls_tjyexqrx0fx5
// Closure Expressions
//
// ferrocene-annotations: fls_jmjn8jkbzujm
// Capturing
//
// ferrocene-annotations: fls_4jiw35pan7vn
// Destruction
//
// ferrocene-annotations: fls_u2mzjgiwbkz0
// Destructors
//
// ferrocene-annotations: fls_rm4ncoopcdvj
// Drop Scopes
//
// ferrocene-annotations: fls_afafmafz4hf2
// Drop Order
//
// ferrocene-annotations: fls_svkx6szhr472
// Ownership
