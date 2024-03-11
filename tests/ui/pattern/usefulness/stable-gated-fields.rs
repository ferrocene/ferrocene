//@ aux-build:unstable.rs

extern crate unstable;

use unstable::UnstableStruct;

fn main() {
    let UnstableStruct { stable } = UnstableStruct::default();
    //~^ pattern does not mention field `stable2` and inaccessible fields

    let UnstableStruct { stable, stable2 } = UnstableStruct::default();
    //~^ pattern requires `..` due to inaccessible fields

    // OK: stable field is matched
    let UnstableStruct { stable, stable2, .. } = UnstableStruct::default();
}

// ferrocene-annotations: fls_7dbd5t2750ce
// Struct Patterns
//
// ferrocene-annotations: fls_nruvg0es3kx7
// Record Struct Patterns
//
// ferrocene-annotations: fls_asj8rgccvkoe
// Struct Pattern Matching
