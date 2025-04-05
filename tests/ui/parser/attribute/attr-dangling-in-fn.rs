fn f() {
  #[foo = "bar"] //~ ERROR expected statement after outer attribute
}

fn main() {
}

// ferrocene-annotations: fls_gvwd0kf72jt
// Attributes
