// https://github.com/rust-lang/rust/issues/77218
//@ run-rustfix
fn main() {
    let value = [7u8];
    while Some(0) = value.get(0) {} //~ ERROR invalid left-hand side of assignment
}

// ferrocene-annotations: fls_y4by2i8dl05o
// Assignment Expressions
// ferrocene-annotations: fls_nnqlae9zp80s
// Basic Assignment
