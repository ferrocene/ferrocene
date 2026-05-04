#![deny(ferrocene::unvalidated)]
//@ check-pass

#![crate_type = "lib"]

use std::fmt::{self, Display};

#[ferrocene::prevalidated]
mod validated {
    pub fn foo() {}

    pub struct Nested;
    impl Clone for Nested {
        fn clone(&self) -> Self {
            Nested
        }
    }

    pub mod double_nested {
        pub fn bar() {}
    }
}

struct Validated;

#[ferrocene::prevalidated]
impl Clone for Validated {
    fn clone(&self) -> Self {
        Validated
    }
}

#[ferrocene::prevalidated]
fn top_level() {
    validated::foo();
    validated::double_nested::bar();
    validated::Nested.clone();
    Validated.clone();
}
