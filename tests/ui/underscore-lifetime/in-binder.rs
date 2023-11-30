// Check that we error when `'_` appears as the name of a lifetime parameter.
//
// Regression test for #52098.

struct IceCube<'a> {
    v: Vec<&'a char>
}

impl<'_> IceCube<'_> {}
//~^ ERROR `'_` cannot be used here

struct Struct<'_> {
    //~^ ERROR `'_` cannot be used here
    v: Vec<&'static char>
}

enum Enum<'_> {
    //~^ ERROR `'_` cannot be used here
    Variant
}

union Union<'_> {
    //~^ ERROR `'_` cannot be used here
    a: u32
}

trait Trait<'_> {
    //~^ ERROR `'_` cannot be used here
}

fn foo<'_>() {
    //~^ ERROR `'_` cannot be used here
}

fn main() {}

// ferrocene-annotations: fls_9ucqbbd0s2yo
// Struct Types
//
// ferrocene-annotations: fls_142vncdktbin
// Reference Types
//
// ferrocene-annotations: fls_fk2m2irwpeof
// Implementations
//
// ferrocene-annotations: fls_szibmtfv117b
// Enum Types
//
// ferrocene-annotations: fls_fmdn7n7s413d
// Union Types
//
// ferrocene-annotations: fls_qcb1n9c0e5hz
// Functions
//
// ferrocene-annotations: fls_9kjpxri0axvg
// Weak Keywords
