trait Zero {
    const ZERO: Self;
}

impl Zero for String {
    const ZERO: Self = String::new();
}

fn foo() {
     match String::new() {
        Zero::ZERO ..= Zero::ZERO => {},
        //~^ ERROR only `char` and numeric types are allowed in range patterns
        _ => {},
    }
}

fn bar() {
    match Zero::ZERO {
        Zero::ZERO ..= Zero::ZERO => {},
        //~^ ERROR type annotations needed [E0282]
        _ => {},
    }
}

fn main() {
    foo();
    bar();
}

// ferrocene-annotations: fls_fyskeih6twyb
// Range Pattern Matching
//
// ferrocene-annotations: fls_6tl1fx99yn6c
// Range Patterns
