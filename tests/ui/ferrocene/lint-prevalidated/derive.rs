#![deny(ferrocene::unvalidated)]
#![feature(const_clone)]
#![feature(derive_const)]

fn main() {
    DeriveValidated.clone(); // ok
    DeriveUnvalidated.clone(); //~ ERROR unvalidated

    DeriveConstValidated.clone(); // ok
    DeriveConstUnvalidated.clone(); //~ ERROR unvalidated
}

#[ferrocene::prevalidated]
#[derive(Clone)]
struct DeriveValidated;

#[derive(Clone)]
struct DeriveUnvalidated;

#[ferrocene::prevalidated]
#[derive_const(Clone)]
struct DeriveConstValidated;

#[derive_const(Clone)]
struct DeriveConstUnvalidated;
