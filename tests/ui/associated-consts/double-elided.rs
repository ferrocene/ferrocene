//@ check-pass

struct S;

impl S {
    const C: &&str = &"";
    // Now resolves to `&'static &'static str`.
}

fn main() {}

// ferrocene-annotations: fls_l9ebxrlxyawd
// Lifetime elision
