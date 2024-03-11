//@ run-pass

fn f(i: &mut Box<isize>) {
    *i = Box::new(200);
}

pub fn main() {
    let mut i = Box::new(100);
    f(&mut i);
    assert_eq!(*i, 200);
}

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
