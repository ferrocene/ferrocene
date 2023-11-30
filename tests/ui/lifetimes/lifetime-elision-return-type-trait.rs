trait Future {
    type Item;
    type Error;
}

use std::error::Error;

fn foo() -> impl Future<Item=(), Error=Box<dyn Error>> {
    //~^ ERROR not satisfied
    Ok(())
}

fn main() {}

// ferrocene-annotations: fls_3xqobbu7wfsf
// Impl Trait Type
//
// ferrocene-annotations: fls_l9ebxrlxyawd
// Lifetime Elision
//
// ferrocene-annotations: fls_hethxxbcg7ja
// Function Lifetime Elision
