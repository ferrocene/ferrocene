//@ run-pass
pub fn main() {
    let i = 5;
    match &&&&i {
        1 ..= 3 => panic!(),
        4 ..= 8 => {},
        _ => panic!(),
    }
}

// ferrocene-annotations: fls_fyskeih6twyb
// Range Pattern Matching
//
// ferrocene-annotations: fls_6tl1fx99yn6c
// Range Patterns
