// Parsing of range patterns

fn main() {
    let macropus!() ..= 11 = 12; //~ error: expected one of `:`, `;`, `=`, or `|`, found `..=`
}

// ferrocene-annotations: fls_6tl1fx99yn6c
// Range Patterns
