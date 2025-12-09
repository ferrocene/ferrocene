// Regression test for #88844.

struct Struct { value: i32 }
//~^ NOTE: similarly named struct `Struct` defined here

impl Stuct {
//~^ ERROR: cannot find type `Stuct` in this scope [E0425]
//~| HELP: a struct with a similar name exists
    fn new() -> Self {
        Self { value: 42 }
    }
}

fn main() {}

// ferrocene-annotations: fls_fk2m2irwpeof
// Implementations
//
// ferrocene-annotations: fls_e1pgdlv81vul
// Implementation Conformance
