//@ dont-require-annotations: NOTE

trait Mumbo {
    fn jumbo(&self, x: &usize) -> usize;
}

impl Mumbo for usize {
    // Cannot have a larger effect than the trait:
    unsafe fn jumbo(&self, x: &usize) { *self + *x; }
    //~^ ERROR method `jumbo` has an incompatible type for trait
    //~| NOTE expected signature `fn
    //~| NOTE found signature `unsafe fn
}

fn main() {}

// ferrocene-annotations: fls_fk2m2irwpeof
// Implementations
//
// ferrocene-annotations: fls_e1pgdlv81vul
// Implementation Conformance
