//@ compile-flags: --test
#![crate_type = "lib"]
#![deny(ferrocene::uncertified)]

fn normal_def() {}

#[cfg(test)]
#[ferrocene::prevalidated]
pub fn validated() {
    normal_def();
    //~^ ERROR unvalidated
    //~^^ ERROR unvalidated
    // TODO: deduplicate this between THIR and post-mono pass
}

#[cfg(test)]
#[test]
fn some_test() {
    normal_def(); // ok: tests assumed unvalidated
}
