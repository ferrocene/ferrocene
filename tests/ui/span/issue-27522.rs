// Point at correct span for self type

struct SomeType {}

trait Foo {
    fn handler(self: &SomeType); //~ ERROR invalid `self` parameter type
}

fn main() {}

// ferrocene-annotations: fls_qcb1n9c0e5hz
// Functions
