// Test that we can't declare a const fn in an impl -- right now it's
// just not allowed at all, though eventually it'd make sense to allow
// it if the trait fn is const (but right now no trait fns can be
// const).

trait Foo {
    fn f() -> u32;
}

impl Foo for u32 {
    const fn f() -> u32 {
        //~^ ERROR functions in trait impls cannot be declared const
        22
    }
}

fn main() {}

// ferrocene-annotations: fls_qcb1n9c0e5hz
// Functions

// ferrocene-annotations: fls_e1pgdlv81vul
// Implementation conformance

// ferrocene-annotations: fls_fk2m2irwpeof
// Implementations

// ferrocene-annotations: fls_85vx1qfa061i
// Traits
