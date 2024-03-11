//@ compile-flags: -Wrust-2021-incompatible-closure-captures

pub struct A {}

impl A {
    async fn create(path: impl AsRef<std::path::Path>)  {
    ;
    crate(move || {} ).await
    }
}


trait C{async fn new(val: T) {}

// ferrocene-annotations: fls_tjyexqrx0fx5
// Closure Expressions
//
// ferrocene-annotations: fls_jmjn8jkbzujm
// Capturing
//
// ferrocene-annotations: um_rustc_W

//~ ERROR this file contains an unclosed delimiter
