#[no_mangle]
pub fn řųśť() {}  //~ `#[no_mangle]` requires ASCII identifier

pub struct Foo;

impl Foo {
    #[no_mangle]
    pub fn řųśť() {}  //~ `#[no_mangle]` requires ASCII identifier
}

trait Bar {
    fn řųśť();
}

impl Bar for Foo {
    #[no_mangle]
    fn řųśť() {}  //~ `#[no_mangle]` requires ASCII identifier
}

fn main() {}

// ferrocene-annotations: fls_21vnag69kbwe
// Identifiers
// ferrocene-annotations: fls_mvd7nz8k3wcy
// Attribute no_mangle
