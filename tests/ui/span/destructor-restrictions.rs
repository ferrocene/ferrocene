// Tests the new destructor semantics.

use std::cell::RefCell;

fn main() {
    let b = {
        let a = Box::new(RefCell::new(4));
        *a.borrow() + 1
    }; //~^ ERROR `*a` does not live long enough
    println!("{}", b);
}

// ferrocene-annotations: fls_yivm43r5wnp1
// Let Statements
//
// ferrocene-annotations: fls_a14slch83hzn
// Borrowing
//
// ferrocene-annotations: fls_5cm4gkt55hjh
// Dereference Expression
//
// ferrocene-annotations: fls_4jiw35pan7vn
// Destruction
