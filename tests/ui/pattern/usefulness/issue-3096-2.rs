enum Bottom { }

fn main() {
    let x = &() as *const () as *const Bottom;
    match x { } //~ ERROR non-exhaustive patterns
}

// ferrocene-annotations: fls_qztk0bkju9u
// Borrow Expression
//
// ferrocene-annotations: fls_a14slch83hzn
// Borrowing
//
// ferrocene-annotations: fls_ppd1xwve3tr7
// Raw Pointer Types
//
// ferrocene-annotations: fls_dw33yt5g6m0k
// Type Coercion
