#[repr(u8)]
enum Alpha {
    V1 = 41,
    V2 = Self::V1 as u8 + 1,    // OK; See #50072.
    V3 = Self::V1 {} as u8 + 2, //~ ERROR cycle detected when simplifying constant
}

fn main() {}

// ferrocene-annotations: fls_6l60b5hwnjbm
// Path Expressions
// ferrocene-annotations: fls_66m4rnbssgig
// Constant Expressions
// ferrocene-annotations: fls_8tsynkj2cufj
// Struct Expressions
