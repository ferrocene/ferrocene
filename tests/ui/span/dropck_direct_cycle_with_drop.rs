// A simple example of an unsound mixing of cyclic structure and Drop.
//
// Each `D` has a name and an optional reference to another `D`
// sibling, but also implements a drop method that prints out its own
// name as well as the name of its sibling.
//
// By setting up a cyclic structure, the drop code cannot possibly
// work. Therefore this code must be rejected.
//
// (As it turns out, essentially any attempt to install a sibling here
//  will be rejected, regardless of whether it forms a cyclic
//  structure or not. This is because the use of the same lifetime
//  `'a` in `&'a D<'a>` cannot be satisfied when `D<'a>` implements
//  `Drop`.)

use std::cell::Cell;

struct D<'a> {
    name: String,
    p: Cell<Option<&'a D<'a>>>,
}

impl<'a> D<'a> {
    fn new(name: String) -> D<'a> { D { name: name, p: Cell::new(None) } }
}

impl<'a> Drop for D<'a> {
    fn drop(&mut self) {
        println!("dropping {} whose sibling is {:?}",
                 self.name, self.p.get().map(|d| &d.name));
    }
}

fn g() {
    let (d1, d2) = (D::new(format!("d1")), D::new(format!("d2")));
    d1.p.set(Some(&d2));
    //~^ ERROR `d2` does not live long enough
    d2.p.set(Some(&d1));
    //~^ ERROR `d1` does not live long enough
}

fn main() {
    g();
}

// ferrocene-annotations: fls_9ucqbbd0s2yo
// Struct Types
//
// ferrocene-annotations: fls_vhpwge5123cm
// Generic Parameters
//
// ferrocene-annotations: fls_yqcygq3y6m5j
// Lifetimes
//
// ferrocene-annotations: fls_3gapgqys3ceb
// Recursive Types
//
// ferrocene-annotations: fls_fk2m2irwpeof
// Implementations
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
