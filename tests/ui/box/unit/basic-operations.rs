//! A collection of very old tests of basic `Box` functionality.
//@ run-pass

fn deref_mut() {
    let mut i: Box<_> = Box::new(0);
    *i = 1;
    assert_eq!(*i, 1);

    // ferrocene-annotations: fls_y4by2i8dl05o
    // Assignment Expressions
    //
    // ferrocene-annotations: fls_nnqlae9zp80s
    // Basic Assignment
    //
    // ferrocene-annotations: fls_5cm4gkt55hjh
    // Dereference Expression
}

// Tests for if as expressions returning boxed types
fn box_if() {
    let rs: Box<_> = if true { Box::new(100) } else { Box::new(101) };
    assert_eq!(*rs, 100);
}

fn cmp() {
    let i: Box<_> = Box::new(100);
    assert_eq!(i, Box::new(100));
    assert!(i < Box::new(101));
    assert!(i <= Box::new(100));
    assert!(i > Box::new(99));
    assert!(i >= Box::new(99));
}

fn autoderef_field() {
    struct J {
        j: isize,
    }

    let i: Box<_> = Box::new(J { j: 100 });
    assert_eq!(i.j, 100);

    // ferrocene-annotations: fls_8tsynkj2cufj
    // Struct Expressions
    //
    // ferrocene-annotations: fls_18k3uajrgq5f
    // Field Access Expressions
    //
    // ferrocene-annotations: fls_xcwfotmq2e5d
    // Field Resolution
}

fn assign_copy() {
    let mut i: Box<_> = Box::new(1);
    // Should be a copy
    let mut j;
    j = i.clone();
    *i = 2;
    *j = 3;
    assert_eq!(*i, 2);
    assert_eq!(*j, 3);

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
}

fn arg_mut() {
    fn f(i: &mut Box<isize>) {
        *i = Box::new(200);
    }
    let mut i = Box::new(100);
    f(&mut i);
    assert_eq!(*i, 200);

    // ferrocene-annotations: fls_xa4nbfas01cj
    // Call Expressions
    //
    // ferrocene-annotations: fls_77scxuomlbgs
    // Passing Conventions
    //
    // ferrocene-annotations: fls_qztk0bkju9u
    // Borrow Expression
    //
    // ferrocene-annotations: fls_a14slch83hzn
    // Borrowing
    //
    // ferrocene-annotations: fls_5cm4gkt55hjh
    // Dereference Expression
}

fn assign_generic() {
    fn f<T>(t: T) -> T {
        let t1 = t;
        t1
    }

    let t = f::<Box<_>>(Box::new(100));
    assert_eq!(t, Box::new(100));

    // ferrocene-annotations: fls_qcb1n9c0e5hz
    // Functions
    //
    // ferrocene-annotations: fls_vhpwge5123cm
    // Generic Parameters
    //
    // ferrocene-annotations: fls_wttihxen35as
    // Constant Promotion
}

pub fn main() {
    deref_mut();
    box_if();
    cmp();
    autoderef_field();
    assign_copy();
    arg_mut();
    assign_generic();
}
