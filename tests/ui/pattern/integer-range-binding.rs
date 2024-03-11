//@ run-pass

fn main() {
    let -2147483648..=2147483647 = 1;
    let 0..=255 = 0u8;
    let -128..=127 = 0i8;
    let '\u{0000}'..='\u{10FFFF}' = 'v';
}

// ferrocene-annotations: fls_azzf1llv3wf
// Literal Pattern Matching
//
// ferrocene-annotations: fls_2krxnq8q9ef1
// Literal Patterns
//
// ferrocene-annotations: fls_fyskeih6twyb
// Range Pattern Matching
//
// ferrocene-annotations: fls_6tl1fx99yn6c
// Range Patterns
