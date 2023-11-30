// Example taken from RFC 1238 text

// https://github.com/rust-lang/rfcs/blob/master/text/1238-nonparametric-dropck.md
//     #examples-of-code-that-will-start-to-be-rejected

// Compare against test/run-pass/issue28498-must-work-ex2.rs

use std::cell::Cell;

struct Concrete<'a>(u32, Cell<Option<&'a Concrete<'a>>>);

struct Foo<T> { data: Vec<T> }

fn potentially_specialized_wrt_t<T>(t: &T) {
    // Hypothetical code that does one thing for generic T and then is
    // specialized for T == Concrete (and the specialized form can
    // then access a reference held in concrete tuple).
    //
    // (We don't have specialization yet, but we want to allow for it
    // in the future.)
}

impl<T> Drop for Foo<T> {
    fn drop(&mut self) {
        potentially_specialized_wrt_t(&self.data[0])
    }
}

fn main() {
    let mut foo = Foo {  data: Vec::new() };
    foo.data.push(Concrete(0, Cell::new(None)));
    foo.data.push(Concrete(0, Cell::new(None)));

    foo.data[0].1.set(Some(&foo.data[1]));
    //~^ ERROR borrow may still be in use when destructor runs
    foo.data[1].1.set(Some(&foo.data[0]));
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
// ferrocene-annotations: fls_142vncdktbin
// Reference Types
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
//
// ferrocene-annotations: fls_qztk0bkju9u
// Borrow Expression
//
// ferrocene-annotations: fls_a14slch83hzn
// Borrowing
