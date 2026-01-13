//@ compile-flags: --test
#![crate_type = "lib"]
#![deny(ferrocene::unvalidated)]

fn normal_def() {}

#[cfg(test)]
#[ferrocene::prevalidated]
pub fn validated() {
    normal_def();
    //~^ ERROR unvalidated
}

#[cfg(test)]
#[test]
fn some_test() {
    normal_def(); // ok: tests are assumed to be unvalidated
}
