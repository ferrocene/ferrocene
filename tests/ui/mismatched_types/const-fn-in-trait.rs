trait Foo {
    fn f() -> u32;
    const fn g(); //~ ERROR cannot be declared const
}

impl Foo for u32 {
    const fn f() -> u32 { 22 } //~ ERROR cannot be declared const
    fn g() {}
}

fn main() { }

// ferrocene-annotations: fls_qcb1n9c0e5hz
// Functions
//
// ferrocene-annotations: fls_85vx1qfa061i
// Traits
