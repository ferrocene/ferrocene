fn main() {
    let mut ref x = 10; //~ ERROR the order of `mut` and `ref` is incorrect
    let ref mut y = 11;
}

// ferrocene-annotations: fls_7bxv8lybxm18
// Identifier Patterns
