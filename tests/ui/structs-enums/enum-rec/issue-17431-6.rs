use std::cell::UnsafeCell;

<<<<<<< HEAD
use std::cell::UnsafeCell;

=======
>>>>>>> pull-upstream-temp--do-not-use-for-real-code
enum Foo { X(UnsafeCell<Option<Foo>>) }
//~^ ERROR recursive type `Foo` has infinite size
//~| ERROR cycle detected

impl Foo { fn bar(self) {} }

fn main() {}
