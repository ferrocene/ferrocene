// Check that safe fns are not a subtype of unsafe fns.

trait Foo {
    unsafe fn len(&self) -> u32;
}

impl Foo for u32 {
    fn len(&self) -> u32 { *self }
    //~^ ERROR method `len` has an incompatible type for trait
    //~| expected signature `unsafe fn(&u32) -> _`
    //~| found signature `fn(&u32) -> _`
}

fn main() { }

// ferrocene-annotations: fls_85vx1qfa061i
// Traits
//
// ferrocene-annotations: fls_fk2m2irwpeof
// Implementations
//
// ferrocene-annotations: fls_e1pgdlv81vul
// Implementation Conformance
