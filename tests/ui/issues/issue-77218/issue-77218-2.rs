<<<PULL-UPSTREAM>>> file deleted upstream; move the Ferrocene annotations if any, and delete this file
//@ run-rustfix
fn main() {
    let value = [7u8];
    while Some(0) = value.get(0) { //~ ERROR invalid left-hand side of assignment
    }
}

// ferrocene-annotations: fls_y4by2i8dl05o
// Assignment Expressions
// ferrocene-annotations: fls_nnqlae9zp80s
// Basic Assignment
//
// ferrocene-annotations: fls_izdv9i4spokw
// Operator Expressions
