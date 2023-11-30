// This basically tests the parser's recovery on `'blk` in the wrong place.

fn foo(cond: bool) {
    let mut x;

    if cond {
        x = &'blk [1,2,3]; //~ ERROR borrow expressions cannot be annotated with lifetimes
    }
}

fn main() {}

// ferrocene-annotations: fls_qztk0bkju9u
// Borrow Expression
//
// ferrocene-annotations: fls_izdv9i4spokw
// Operator Expressions
