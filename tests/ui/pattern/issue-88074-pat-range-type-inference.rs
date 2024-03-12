//@ check-pass

trait Zero {
    const ZERO: Self;
}

impl Zero for i32 {
    const ZERO: Self = 0;
}

fn main() {
    match 1 {
        Zero::ZERO ..= 1 => {},
        _ => {},
    }
}

// ferrocene-annotations: fls_fyskeih6twyb
// Range Pattern Matching
//
// ferrocene-annotations: fls_6tl1fx99yn6c
// Range Patterns
