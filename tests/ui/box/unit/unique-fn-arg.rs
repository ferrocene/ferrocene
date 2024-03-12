//@ run-pass

fn f(i: Box<isize>) {
    assert_eq!(*i, 100);
}

pub fn main() {
    f(Box::new(100));
    let i = Box::new(100);
    f(i);
}

// ferrocene-annotations: fls_xa4nbfas01cj
// Call Expressions
//
// ferrocene-annotations: fls_77scxuomlbgs
// Passing Conventions
