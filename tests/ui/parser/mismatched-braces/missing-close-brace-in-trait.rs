trait T {
    fn foo(&self);

pub(crate) struct Bar<T>();

impl T for Bar<usize> {

fn foo(&self) {}
}

// ferrocene-annotations: fls_85vx1qfa061i
// Traits

fn main() {} //~ ERROR this file contains an unclosed delimiter
