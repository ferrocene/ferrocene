//@ edition:2021

mod m {
  pub struct S { foo: i32 }
  impl S {
    pub fn foo(&self) -> i32 { 42 }
  }
}

fn bar(s: &m::S) {
  || s.foo() + s.foo; //~ ERROR E0616
}

fn main() {}

// ferrocene-annotations: fls_jdknpu3kf865
// Visibility
//
// ferrocene-annotations: fls_9ucqbbd0s2yo
// Struct Types
//
// ferrocene-annotations: fls_18k3uajrgq5f
// Field Access Expressions
//
// ferrocene-annotations: fls_xcwfotmq2e5d
// Field Resolution
