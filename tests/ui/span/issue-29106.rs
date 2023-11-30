use std::rc::Rc;
use std::sync::Arc;

struct Foo<'a>(&'a String);

impl<'a> Drop for Foo<'a> {
    fn drop(&mut self) {
        println!("{:?}", self.0);
    }
}

fn main() {
    {
        let (y, x);
        x = "alive".to_string();
        y = Arc::new(Foo(&x));
    }
    //~^^ ERROR `x` does not live long enough

    {
        let (y, x);
        x = "alive".to_string();
        y = Rc::new(Foo(&x));
    }
    //~^^ ERROR `x` does not live long enough
}

// ferrocene-annotations: fls_9ucqbbd0s2yo
// Struct Types
//
// ferrocene-annotations: fls_fk2m2irwpeof
// Implementations
//
// ferrocene-annotations: fls_vhpwge5123cm
// Generic Parameters
//
// ferrocene-annotations: fls_yqcygq3y6m5j
// Lifetimes
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
