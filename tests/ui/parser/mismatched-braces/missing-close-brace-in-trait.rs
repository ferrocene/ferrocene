trait T {
    fn foo(&self);

pub(crate) struct Bar<T>();
//~^ ERROR struct is not supported in `trait`s or `impl`s

impl T for Bar<usize> {
//~^ ERROR implementation is not supported in `trait`s or `impl`s
fn foo(&self) {}
}

// ferrocene-annotations: fls_85vx1qfa061i
// Traits

fn main() {} //~ ERROR this file contains an unclosed delimiter
