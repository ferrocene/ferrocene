trait Foo {
    fn bar(&self, x: usize) -> Self;
}
impl Foo for isize {
    fn bar(&self) -> isize {
        //~^ ERROR method `bar` has 1 parameter but the declaration in trait `Foo::bar` has 2
        *self
    }
}

fn main() {
}

// ferrocene-annotations: fls_e1pgdlv81vul
// Implementation Conformance
//
// ferrocene-annotations: fls_fk2m2irwpeof
// Implementations
