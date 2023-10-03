use std::cell::RefCell;

fn main() {
    let m = RefCell::new(0);
    let p;
    {
        let b = m.borrow();
        p = &*b;
    }
    //~^^ ERROR `b` does not live long enough
    p.use_ref();
}

trait Fake { fn use_mut(&mut self) { } fn use_ref(&self) { }  }
impl<T> Fake for T { }

// ferrocene-annotations: fls_a14slch83hzn
// Borrowing
//
// ferrocene-annotations: fls_qztk0bkju9u
// Borrow Expression
//
// ferrocene-annotations: fls_5cm4gkt55hjh
// Dereference Expression
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
// ferrocene-annotations: fls_izdv9i4spokw
// Operator Expressions
