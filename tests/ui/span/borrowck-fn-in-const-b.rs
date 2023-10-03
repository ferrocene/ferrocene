// Check that we check fns appearing in constant declarations.
// Issue #22382.

// How about mutating an immutable vector?
const MUTATE: fn(&Vec<String>) = {
    fn broken(x: &Vec<String>) {
        x.push(format!("this is broken"));
        //~^ ERROR cannot borrow
    }
    broken
};

fn main() {
}

// ferrocene-annotations: fls_ixjc5jaamx84
// Constants
//
// ferrocene-annotations: fls_142vncdktbin
// Reference Types
//
// ferrocene-annotations: fls_a14slch83hzn
// Borrowing
