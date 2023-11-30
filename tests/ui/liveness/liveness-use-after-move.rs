fn main() {

    let x: Box<_> = 5.into();
    let y = x;

    println!("{}", *x); //~ ERROR borrow of moved value: `x`
    y.clone();
}

// ferrocene-annotations: fls_a14slch83hzn
// Borrowing
//
// ferrocene-annotations: fls_77scxuomlbgs
// Passing Conventions
