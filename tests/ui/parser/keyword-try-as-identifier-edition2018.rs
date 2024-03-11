//@ compile-flags: --edition 2018

fn main() {
    let try = "foo"; //~ error: expected identifier, found reserved keyword `try`
}

// ferrocene-annotations: fls_lish33a1naw5
// Keywords
//
// ferrocene-annotations: fls_cbsgp6k0qa82
// Reserved Keywords
