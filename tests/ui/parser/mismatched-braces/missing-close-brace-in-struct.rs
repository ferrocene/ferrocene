pub(crate) struct Bar<T> {
  foo: T,

trait T {
    fn foo(&self);
}


impl T for Bar<usize> {
fn foo(&self) {}
}

// ferrocene-annotations: fls_9ucqbbd0s2yo
// Struct Types

fn main() {} //~ ERROR this file contains an unclosed delimiter
