// Test that cross-borrowing (implicitly converting from `Box<T>` to `&T`) is
// forbidden when `T` is a trait.

//@ dont-require-annotations: NOTE

struct Foo;
trait Trait { fn foo(&self) {} }
impl Trait for Foo {}

pub fn main() {
    let x: Box<dyn Trait> = Box::new(Foo);
    let _y: &dyn Trait = x; //~ ERROR E0308
                            //~| NOTE expected reference `&dyn Trait`
                            //~| NOTE found struct `Box<dyn Trait>`
}

// ferrocene-annotations: fls_a14slch83hzn
// Borrowing
//
// ferrocene-annotations: fls_qa98qdi42orq
// Trait Object Type
