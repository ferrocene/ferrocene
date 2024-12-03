//@ run-pass
#![allow(dead_code)]
#![allow(non_camel_case_types)]



struct kitten {
    cat: Option<cat>,
}

fn kitten(cat: Option<cat>) -> kitten {
    kitten {
        cat: cat
    }
}

type cat = Box<kitten>;

pub fn main() {}

// ferrocene-annotations: fls_9ucqbbd0s2yo
// Struct Types
//
// ferrocene-annotations: fls_3gapgqys3ceb
// Recursive Types
