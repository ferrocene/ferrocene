//@ error-pattern:expected statement

fn f() {
  #[foo = "bar"]
}

fn main() {
}

// ferrocene-annotations: fls_gvwd0kf72jt
// Attributes
