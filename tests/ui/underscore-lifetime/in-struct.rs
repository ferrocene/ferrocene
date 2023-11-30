// Check that the `'_` used in structs/enums gives an error.

use std::fmt::Debug;

struct Foo {
    x: &'_ u32, //~ ERROR missing lifetime specifier
}

enum Bar {
    Variant(&'_ u32), //~ ERROR missing lifetime specifier
}

fn main() { }

// ferrocene-annotations: fls_9ucqbbd0s2yo
// Struct Types
//
// ferrocene-annotations: fls_szibmtfv117b
// Enum Types
//
// ferrocene-annotations: fls_142vncdktbin
// Reference Types
