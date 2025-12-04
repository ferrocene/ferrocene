//@ edition:2015
fn foo<T>() where <T>::Item: ToString, T: Iterator { }
//~^ ERROR generic parameters on `where` clauses are reserved for future use
//~| ERROR cannot find type `Item` in the crate root

fn main() {}

// ferrocene-annotations: fls_7nv8ualeaqe3
// Where Clauses
