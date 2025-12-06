//@ edition:2015
fn main() {
    ::foo() //~ ERROR cannot find external crate `foo` in the crate root
}

// ferrocene-annotations: fls_maw4u1o8q37u
// Crates
