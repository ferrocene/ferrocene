struct Foo {
    x: isize,
}

impl Foo {
    pub fn f(&self) {}
    pub fn h(&mut self) {}
}

fn a(x: &mut Foo) {
    x.f();
    x.h();
}

fn b(x: &Foo) {
    x.f();
    x.h(); //~ ERROR cannot borrow
}

fn main() {
}

// ferrocene-annotations: fls_9ucqbbd0s2yo
// Struct Types
//
// ferrocene-annotations: fls_fk2m2irwpeof
// Implementations
//
// ferrocene-annotations: fls_142vncdktbin
// Reference Types
//
// ferrocene-annotations: fls_a14slch83hzn
// Borrowing
//
// ferrocene-annotations: fls_xa4nbfas01cj
// Call Expressions
