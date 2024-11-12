//@ run-pass
#![allow(unused_variables)]
// Check that a trait is still dyn-compatible (and usable) if it has
// generic methods so long as they require `Self : Sized`.


trait Counter {
    fn tick(&mut self) -> u32;
    fn with<F:FnOnce(u32)>(&self, f: F) where Self : Sized;
}

struct CCounter {
    c: u32
}

impl Counter for CCounter {
    fn tick(&mut self) -> u32 { self.c += 1; self.c }
    fn with<F:FnOnce(u32)>(&self, f: F) { f(self.c); }
}

fn tick1<C:Counter>(c: &mut C) {
    tick2(c);
    c.with(|i| ());
}

fn tick2(c: &mut dyn Counter) {
    tick3(c);
}

fn tick3<C:?Sized+Counter>(c: &mut C) {
    c.tick();
    c.tick();
}

fn main() {
    let mut c = CCounter { c: 0 };
    tick1(&mut c);
    assert_eq!(c.tick(), 3);
}

// ferrocene-annotations: fls_7nv8ualeaqe3
// Where Clauses
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
// ferrocene-annotations: fls_77scxuomlbgs
// Passing Conventions
//
// ferrocene-annotations: fls_jeoas4n6su4
// Trait and Lifetime Bounds
