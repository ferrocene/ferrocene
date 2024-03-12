//@ run-rustfix
//@ check-pass

#![warn(rust_2018_compatibility)]

fn main() {
    try();
    //~^ WARNING `try` is a keyword in the 2018 edition
    //~| WARNING this is accepted in the current edition
}

fn try() {
    //~^ WARNING `try` is a keyword in the 2018 edition
    //~| WARNING this is accepted in the current edition
}

// ferrocene-annotations: fls_lish33a1naw5
// Keywords
//
// ferrocene-annotations: fls_cbsgp6k0qa82
// Reserved Keywords
