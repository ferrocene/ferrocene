trait Foo {
    fn foo();
}

struct Bar;

impl Foo for Bar {}
//~^ ERROR E0046

fn main() {
}

// ferrocene-annotations: fls_e1pgdlv81vul
// Implementation Conformance
