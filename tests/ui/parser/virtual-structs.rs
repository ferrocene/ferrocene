// Test diagnostics for the removed struct inheritance feature.

virtual struct SuperStruct {
//~^ ERROR expected item, found reserved keyword `virtual`
    f1: isize,
}

struct Struct : SuperStruct;

pub fn main() {}

// ferrocene-annotations: fls_lish33a1naw5
// Keywords
//
// ferrocene-annotations: fls_cbsgp6k0qa82
// Reserved Keywords
