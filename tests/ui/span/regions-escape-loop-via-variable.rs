fn main() {
    let x = 3;

    // Here, the variable `p` gets inferred to a type with a lifetime
    // of the loop body.  The regionck then determines that this type
    // is invalid.
    let mut p = &x;

    loop {
        let x = 1 + *p;
        p = &x;
    }
    //~^^ ERROR `x` does not live long enough
}

// ferrocene-annotations: fls_a14slch83hzn
// Borrowing
//
// ferrocene-annotations: fls_qztk0bkju9u
// Borrow Expression
//
// ferrocene-annotations: fls_5cm4gkt55hjh
// Dereference Expression
//
// ferrocene-annotations: fls_svkx6szhr472
// Ownership
