// Test that `dyn Bar<Item = XX>` uses `'static` as the default object
// lifetime bound for the type `XX`.
//
//@ check-pass

trait Foo {
    type Item: ?Sized;

    fn item(&self) -> Box<Self::Item> { panic!() }
}

trait Bar { }

impl<T> Foo for T {
    type Item = dyn Bar;
}

fn is_static<T>(_: T) where T: 'static { }

// Here, we default to `dyn Bar + 'static`, and not `&'x dyn Foo<Item
// = dyn Bar + 'x>`.
fn bar(x: &str) -> &dyn Foo<Item = dyn Bar> { &() }

fn main() {
    let s = format!("foo");
    let r = bar(&s);
    is_static(r.item());
}

// ferrocene-annotations: fls_vhpwge5123cm
// Generic Parameters
//
// ferrocene-annotations: fls_qa98qdi42orq
// Trait Object Types
//
// ferrocene-annotations: fls_7nv8ualeaqe3
// Where Clauses
