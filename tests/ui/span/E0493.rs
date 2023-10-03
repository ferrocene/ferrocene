struct Foo {
    a: u32
}

impl Drop for Foo {
    fn drop(&mut self) {}
}

struct Bar {
    a: u32
}

impl Drop for Bar {
    fn drop(&mut self) {}
}

const F : Foo = (Foo { a : 0 }, Foo { a : 1 }).1;
//~^ ERROR destructor of

fn main() {
}

// ferrocene-annotations: fls_4jiw35pan7vn
// Destruction
//
// ferrocene-annotations: fls_u2mzjgiwbkz0
// Destructors
