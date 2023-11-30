fn foo<T:'static>() {
    1.bar::<T>(); //~ ERROR `T` cannot be sent between threads safely
}

trait Bar {
    fn bar<T:Send>(&self);
}

impl Bar for usize {
    fn bar<T:Send>(&self) {
    }
}

fn main() {}

// ferrocene-annotations: fls_fk2m2irwpeof
// Implementations
//
// ferrocene-annotations: fls_jeoas4n6su4
// Trait and Lifetime Bounds
