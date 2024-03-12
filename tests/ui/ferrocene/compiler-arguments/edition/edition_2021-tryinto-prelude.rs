//@ check-pass
//@ compile-flags:--extern some_crate --edition 2021

struct Foo;

impl TryInto<String> for Foo {
    type Error = ();
    fn try_into(self) -> Result<String, ()> {
        todo!()
    }
}

fn main() {}

// ferrocene-annotations: um_rustc_edition
