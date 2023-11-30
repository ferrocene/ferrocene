fn main() {
    let x = "\x80"; //~ ERROR out of range hex escape
    let y = "\xff"; //~ ERROR out of range hex escape
    let z = "\xe2"; //~ ERROR out of range hex escape
    let a = b"\x00e2";  // ok because byte literal
}

// ferrocene-annotations: fls_ypa86oqxhn9u
// Character Literals
//
// ferrocene-annotations: fls_msbaxfc09vkk
// Simple Byte String Literals
//
// ferrocene-annotations: fls_h0dvogc64tfh
// Literal Expressions
//
// ferrocene-annotations: fls_e7zgqroy2qxn
// Value Expressions
