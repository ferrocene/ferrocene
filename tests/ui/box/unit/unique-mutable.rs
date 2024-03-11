//@ run-pass

pub fn main() {
    let mut i: Box<_> = Box::new(0);
    *i = 1;
    assert_eq!(*i, 1);
}

// ferrocene-annotations: fls_y4by2i8dl05o
// Assignment Expressions
//
// ferrocene-annotations: fls_nnqlae9zp80s
// Basic Assignment
//
// ferrocene-annotations: fls_5cm4gkt55hjh
// Dereference Expression
