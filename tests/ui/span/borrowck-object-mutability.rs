trait Foo {
    fn borrowed(&self);
    fn borrowed_mut(&mut self);
}

fn borrowed_receiver(x: &dyn Foo) {
    x.borrowed();
    x.borrowed_mut(); //~ ERROR cannot borrow
}

fn borrowed_mut_receiver(x: &mut dyn Foo) {
    x.borrowed();
    x.borrowed_mut();
}

fn owned_receiver(x: Box<dyn Foo>) {
    x.borrowed();
    x.borrowed_mut(); //~ ERROR cannot borrow
}

fn mut_owned_receiver(mut x: Box<dyn Foo>) {
    x.borrowed();
    x.borrowed_mut();
}

fn main() {}

// ferrocene-annotations: fls_85vx1qfa061i
// Traits
//
// ferrocene-annotations: fls_142vncdktbin
// Reference Types
//
// ferrocene-annotations: fls_qcb1n9c0e5hz
// Functions
//
// ferrocene-annotations: fls_z7q8kbjwdc7g
// Method Call Expressions
//
// ferrocene-annotations: fls_qa98qdi42orq
// Trait Object Types
//
// ferrocene-annotations: fls_wqazkzle0ix9
// Method Resolution
