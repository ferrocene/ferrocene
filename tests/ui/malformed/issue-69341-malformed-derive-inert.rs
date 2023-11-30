fn main() {}

struct CLI {
    #[derive(parse())] //~ ERROR expected non-macro attribute, found attribute macro
    path: (),
}

// ferrocene-annotations: fls_r6gj1p4gajnq
// Attribute derive
