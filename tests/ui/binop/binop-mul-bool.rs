//@ error-pattern:cannot multiply `bool` by `bool`

fn main() { let x = true * false; }

// ferrocene-annotations: fls_tiqp1gxf116z
// Bool Type
//
// ferrocene-annotations: fls_1k9mkv7rbezi
// Arithmetic Expressions
//
// ferrocene-annotations: fls_zfibijmf8qe1
// Arithmetic Overflow
