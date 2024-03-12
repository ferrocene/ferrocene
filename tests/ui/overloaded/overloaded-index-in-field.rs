//@ run-pass
// Test using overloaded indexing when the "map" is stored in a
// field. This caused problems at some point.

use std::ops::Index;

struct Foo {
    x: isize,
    y: isize,
}

struct Bar {
    foo: Foo
}

impl Index<isize> for Foo {
    type Output = isize;

    fn index(&self, z: isize) -> &isize {
        if z == 0 {
            &self.x
        } else {
            &self.y
        }
    }
}

trait Int {
    fn get(self) -> isize;
    fn get_from_ref(&self) -> isize; //~ WARN methods `get_from_ref` and `inc` are never used
    fn inc(&mut self);
}

impl Int for isize {
    fn get(self) -> isize { self }
    fn get_from_ref(&self) -> isize { *self }
    fn inc(&mut self) { *self += 1; }
}

fn main() {
    let f = Bar { foo: Foo {
        x: 1,
        y: 2,
    } };
    assert_eq!(f.foo[1].get(), 2);
}

// ferrocene-annotations: fls_qztk0bkju9u
// Borrow Expression
//
// ferrocene-annotations: fls_vhpwge5123cm
// Generic Parameters
//
// ferrocene-annotations: fls_fk2m2irwpeof
// Implementations
//
// ferrocene-annotations: fls_9ucqbbd0s2yo
// Struct Types
