use std::ops::Deref;

struct NotCopy {
    inner: bool
}

impl NotCopy {
    fn inner_method(&self) {}
}

struct Foo {
    first: NotCopy,
    second: NotCopy
}

impl Deref for Foo {
    type Target = NotCopy;
    fn deref(&self) -> &NotCopy {
        &self.second
    }
}

fn use_field(val: Foo) {
    let _val = val.first;
    val.inner; //~ ERROR borrow of
}

fn use_method(val: Foo) {
    let _val = val.first;
    val.inner_method(); //~ ERROR borrow of
}

fn main() {}

// ferrocene-annotations: fls_z7q8kbjwdc7g
// Method Call Expressions
//
// ferrocene-annotations: fls_18k3uajrgq5f
// Field Access Expressions
//
// ferrocene-annotations: fls_6ydylimiv553
// Place Expressions
//
// ferrocene-annotations: fls_xcwfotmq2e5d
// Field Resolution
//
// ferrocene-annotations: fls_wqazkzle0ix9
// Method Resolution
