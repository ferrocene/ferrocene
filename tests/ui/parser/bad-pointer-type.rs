fn foo(_: *()) {
    //~^ ERROR expected `mut` or `const` keyword in raw pointer type
}

fn main() {}

// ferrocene-annotations: fls_ppd1xwve3tr7
// Raw Pointer Types
