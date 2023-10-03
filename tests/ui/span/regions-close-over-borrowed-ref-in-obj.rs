fn id<T>(x: T) -> T { x }

trait Foo { }

impl<'a> Foo for &'a isize { }

fn main() {

    let blah;

    {
        let ss: &isize = &id(1);
        //~^ ERROR temporary value dropped while borrowed
        blah = Box::new(ss) as Box<dyn Foo>;
    }
}

// ferrocene-annotations: fls_142vncdktbin
// Reference Types
//
// ferrocene-annotations: fls_qztk0bkju9u
// Borrow Expression
//
// ferrocene-annotations: fls_a14slch83hzn
// Borrowing
//
// ferrocene-annotations: fls_cleoffpn5ew6
// Temporaries
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
//
// ferrocene-annotations: fls_gho955gmob73
// Variables
