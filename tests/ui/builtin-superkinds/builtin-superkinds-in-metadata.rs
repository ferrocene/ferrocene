//@ aux-build:trait_superkinds_in_metadata.rs

// Test for traits inheriting from the builtin kinds cross-crate.
// Mostly tests correctness of metadata.

extern crate trait_superkinds_in_metadata;
use trait_superkinds_in_metadata::{RequiresRequiresShareAndSend, RequiresShare};

struct X<T>(T);

impl <T:Sync> RequiresShare for X<T> { }

impl <T:Sync+'static> RequiresRequiresShareAndSend for X<T> { }
//~^ ERROR `T` cannot be sent between threads safely [E0277]

fn main() { }

// ferrocene-annotations: fls_jeoas4n6su4
// Trait and Lifetime Bounds
//
// ferrocene-annotations: fls_gklst7joeo33
// External Crates
//
// ferrocene-annotations: fls_fk2m2irwpeof
// Implementations
//
// ferrocene-annotations: fls_eiw4by8z75di
// Send and Sync
