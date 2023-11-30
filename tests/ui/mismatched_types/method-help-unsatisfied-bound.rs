struct Foo;

fn main() {
    let a: Result<(), Foo> = Ok(());
    a.unwrap();
    //~^ ERROR `Foo` doesn't implement `Debug`
}

// ferrocene-annotations: fls_jeoas4n6su4
// Trait and Lifetime Bounds
