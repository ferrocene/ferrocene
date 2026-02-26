<<<PULL-UPSTREAM>>> file deleted upstream; move the Ferrocene annotations if any, and delete this file
//@ run-pass

pub fn main() {
    let i: Box<_> = Box::new(100);
    let j = i;
    assert_eq!(*j, 100);
}

// ferrocene-annotations: fls_77scxuomlbgs
// Passing Conventions
