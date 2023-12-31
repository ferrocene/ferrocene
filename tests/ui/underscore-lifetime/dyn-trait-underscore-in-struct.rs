// Check that the `'_` in `dyn Trait + '_` acts like ordinary elision,
// and not like an object lifetime default.
//
// cc #48468

use std::fmt::Debug;

struct Foo {
    x: Box<dyn Debug + '_>, //~ ERROR missing lifetime specifier
}

fn main() {}

// ferrocene-annotations: fls_9ucqbbd0s2yo
// Struct Types
//
// ferrocene-annotations: fls_qa98qdi42orq
// Trait Object Types
