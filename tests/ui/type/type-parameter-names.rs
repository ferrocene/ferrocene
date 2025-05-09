// Test that we print out the names of type parameters correctly in
// our error messages.

//@ dont-require-annotations: NOTE

fn foo<Foo, Bar>(x: Foo) -> Bar {
    x
//~^ ERROR mismatched types
//~| NOTE expected type parameter `Bar`, found type parameter `Foo`
//~| NOTE expected type parameter `Bar`
//~| NOTE found type parameter `Foo`
}

fn main() {}

// ferrocene-annotations: fls_vhpwge5123cm
// Generic Parameters
// ferrocene-annotations: fls_exe4zodlwfez
// Type Unification
