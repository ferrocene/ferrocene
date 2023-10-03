// Test calling methods on an impl for a bare trait. This test checks that the
// trait impl is only applied to a trait object, not concrete types which implement
// the trait.

trait T {}

impl<'a> dyn T + 'a {
    fn foo(&self) {}
}

impl T for i32 {}

fn main() {
    let x = &42i32;
    x.foo(); //~ERROR: no method named `foo` found
}

// ferrocene-annotations: fls_qa98qdi42orq
// Trait Object Types
//
// ferrocene-annotations: fls_z7q8kbjwdc7g
// Method Call Expressions
//
// ferrocene-annotations: fls_wqazkzle0ix9
// Method Resolution
