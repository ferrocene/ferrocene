// A method's receiver must be well-formed, even if it has late-bound regions.
// Because of this, a method's substs being well-formed does not imply that
// the method's implied bounds are met.

struct Foo<'b>(Option<&'b ()>);

trait Bar<'b> {
    fn xmute<'a>(&'a self, u: &'b u32) -> &'a u32;
}

impl<'b> Bar<'b> for Foo<'b> {
    fn xmute<'a>(&'a self, u: &'b u32) -> &'a u32 { u }
}

fn main() {
    let f = Foo(None);
    let f2 = f;
    let dangling = {
        let pointer = Box::new(42);
        f2.xmute(&pointer)
    };
    //~^^ ERROR `pointer` does not live long enough
    println!("{}", dangling);
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
// ferrocene-annotations: fls_fk2m2irwpeof
// Implementations
//
// ferrocene-annotations: fls_e1pgdlv81vul
// Implementation Conformance
//
// ferrocene-annotations: fls_qztk0bkju9u
// Borrow Expression
//
// ferrocene-annotations: fls_a14slch83hzn
// Borrowing
//
// ferrocene-annotations: fls_svkx6szhr472
// Ownership
