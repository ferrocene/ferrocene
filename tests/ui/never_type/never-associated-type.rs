// Test that we can use ! as an associated type.

//@ check-pass

#![feature(never_type)]

trait Foo {
    type Wow;

    fn smeg(&self) -> Self::Wow;
}

struct Blah;
impl Foo for Blah {
    type Wow = !;
    fn smeg(&self) -> ! {
        panic!("kapow!");
    }
}

fn main() {
    Blah.smeg();
}

// ferrocene-annotations: fls_e1pgdlv81vul
// Implementation Conformance
