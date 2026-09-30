//! Regression test for <https://github.com/rust-lang/rust/issues/21763>.
//! Test HashMap only impl Send/Sync if its contents do

//@ normalize-stderr: "(?:[A-Za-z]:[/\\]|/).*[\\/]hashbrown\S+" -> "$$HASHBROWN_SRC_LOCATION"

// Ferrocene addition: Our CI automatically remaps the source path to start with $BUILD_DIR.
// So the above substitution ends up as $BUILD_DIR$HASHBROWN_SRC_LOCATION, which differs from
// what the test expects. Delete the extra tag.
//@ normalize-stderr: "\$BUILD_DIR" -> ""

use std::collections::HashMap;
use std::rc::Rc;

fn foo<T: Send>() {}

fn main() {
    foo::<HashMap<Rc<()>, Rc<()>>>();
    //~^ ERROR `Rc<()>` cannot be sent between threads safely
}
