fn main() {
    struct Foo { x: isize }
    match (Foo { x: 10 }) {
        Foo { ref x: ref x } => {}, //~ ERROR expected `,`
        _ => {}
    }
}

// ferrocene-annotations: fls_nruvg0es3kx7
// Record Struct Patterns
//
// ferrocene-annotations: fls_7dbd5t2750ce
// Struct Patterns
