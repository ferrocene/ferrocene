//@ check-fail
//@ compile-flags:--extern some_crate --edition 2018

struct Foo;

impl TryInto<String> for Foo { //~ ERROR [E0405]
    type Error = ();
    fn try_into(self) -> Result<String, ()> {
        todo!()
    }
}

fn main() {}

// ferrocene-annotations: um_rustc_edition
