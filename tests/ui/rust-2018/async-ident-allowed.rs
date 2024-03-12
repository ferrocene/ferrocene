//@ edition:2015

#![deny(rust_2018_compatibility)]

// Don't make a suggestion for a raw identifier replacement unless raw
// identifiers are enabled.

fn main() {
    let async = 3; //~ ERROR: is a keyword
    //~^ WARN this is accepted in the current edition
}

// ferrocene-annotations: fls_lish33a1naw5
// Keywords
//
// ferrocene-annotations: fls_mec5cg5aptf8
// Strict Keywords
//
// ferrocene-annotations: fls_21vnag69kbwe
// Identifiers
