enum E {
    A(u8, u8),
}

fn main() {
    let e = E::A(2, 3);
    match e {
        E::A(x @ ..) => {
            //~^ ERROR: `x @` is not allowed in a tuple struct
            //~| ERROR: `..` patterns are not allowed here
            //~| ERROR: this pattern has 1 field, but the corresponding tuple variant has 2 fields
            x
        }
    };
}

// ferrocene-annotations: fls_vnai6ag4qrdb
// Identifier Pattern Matching
//
// ferrocene-annotations: fls_7bxv8lybxm18
// Identifier Patterns
//
// ferrocene-annotations: fls_eexupzdsu7f
// Tuple Struct Pattern Matching
//
// ferrocene-annotations: fls_vlrto778v49m
// Tuple Struct Patterns
