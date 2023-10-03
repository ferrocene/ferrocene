// Test range syntax - borrow errors.
#![feature(rustc_attrs)]
pub fn main() { #![rustc_error] // rust-lang/rust#49855
    let r = {
        let a = 42;
        let b = 42;
        &a..&b
    };
    //~^^ ERROR `a` does not live long enough
    //~| ERROR `b` does not live long enough
    r.use_ref();
}

trait Fake { fn use_mut(&mut self) { } fn use_ref(&self) { }  }
impl<T> Fake for T { }

// ferrocene-annotations: fls_a14slch83hzn
// Borrowing
//
// ferrocene-annotations: fls_qztk0bkju9u
// Borrow Expression
//
// ferrocene-annotations: fls_18swodqqzadj
// Range Expressions
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
