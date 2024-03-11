//@ run-pass

pub fn main() {
    let mut i: Box<_> = Box::new(1);
    // Should be a copy
    let mut j = i.clone();
    *i = 2;
    *j = 3;
    assert_eq!(*i, 2);
    assert_eq!(*j, 3);
}

// ferrocene-annotations: fls_77scxuomlbgs
// Passing Conventions
//
// ferrocene-annotations: fls_5cm4gkt55hjh
// Dereference Expression
//
// ferrocene-annotations: fls_y4by2i8dl05o
// Assignment Expressions
//
// ferrocene-annotations: fls_nnqlae9zp80s
// Basic Assignment
//
// ferrocene-annotations: fls_izdv9i4spokw
// Operator Expressions
