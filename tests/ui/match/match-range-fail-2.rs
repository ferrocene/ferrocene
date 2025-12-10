fn main() {
    match 5 {
        6 ..= 1 => { }
        //~^ ERROR lower bound for range pattern must be less than or equal to upper bound
        _ => { }
    };

    match 5 {
        0 .. 0 => { }
        //~^ ERROR lower bound for range pattern must be less than upper bound
        _ => { }
    };

    match 5u64 {
        0xFFFF_FFFF_FFFF_FFFF ..= 1 => { }
        //~^ ERROR lower bound for range pattern must be less than or equal to upper bound
        _ => { }
    };
}

// ferrocene-annotations: fls_fyskeih6twyb
// Range Pattern Matching
//
// ferrocene-annotations: fls_6tl1fx99yn6c
// Range Patterns
