use std::rc::Rc;

trait Foo {
    fn foo(self: &Rc<Self>) -> usize;
}

trait Bar {
    fn foo(self: &Rc<Self>) -> usize where Self: Sized;
    fn bar(self: Rc<Self>) -> usize;
}

impl Foo for usize {
    fn foo(self: &Rc<Self>) -> usize {
        **self
    }
}

impl Bar for usize {
    fn foo(self: &Rc<Self>) -> usize {
        **self
    }

    fn bar(self: Rc<Self>) -> usize {
        *self
    }
}

fn make_foo() {
    let x = Rc::new(5usize) as Rc<dyn Foo>;
    //~^ ERROR E0038
}

fn make_bar() {
    let x = Rc::new(5usize) as Rc<dyn Bar>;
    x.bar();
}

fn main() {}

// ferrocene-annotations: fls_4ikc07mfrez5
// Object Safety
//
// ferrocene-annotations: fls_142vncdktbin
// Reference Types
//
// ferrocene-annotations: fls_5cm4gkt55hjh
// Dereference Expression
