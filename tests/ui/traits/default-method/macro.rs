//@ run-pass


trait Foo {
    fn bar(&self) -> String {
        format!("test")
    }
}

enum Baz {
    Quux
}

impl Foo for Baz {
}

pub fn main() {
    let q = Baz::Quux;
    assert_eq!(q.bar(), "test".to_string());
}

// ferrocene-annotations: fls_z7q8kbjwdc7g
// Method Call Expressions
//
// ferrocene-annotations: fls_jeoas4n6su4
// Trait and Lifetime Bounds
//
// ferrocene-annotations: fls_wqazkzle0ix9
// Method Resolution
