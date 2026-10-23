//! Regression test for <https://github.com/rust-lang/rust/issues/21763>.
//! Test HashMap only impl Send/Sync if its contents do

//@ normalize-stderr: "(?:\$BUILD_DIR|[A-Za-z]:[/\\]|/).*[\\/]hashbrown\S+" -> "$$HASHBROWN_SRC_LOCATION"

use std::collections::HashMap;
use std::rc::Rc;

fn foo<T: Send>() {}

fn main() {
    foo::<HashMap<Rc<()>, Rc<()>>>();
    //~^ ERROR `Rc<()>` cannot be sent between threads safely
}

// Ferrocene change: in CI, stderr normalization can replace the first of the
// hashbrown path with `$BUILD_DIR`, which breaks this test. We fix it by
// matching `$BUILD_DIR` as part of the custom normalization rule above.
