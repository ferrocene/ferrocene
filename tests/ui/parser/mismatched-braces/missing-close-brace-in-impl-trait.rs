fn main() {}

impl T for () { //~ ERROR cannot find trait `T` in this scope

fn foo(&self) {}

trait T { //~ ERROR trait is not supported in `trait`s or `impl`s
    fn foo(&self);
}

pub(crate) struct Bar<T>(); //~ ERROR struct is not supported in `trait`s or `impl`s

// ferrocene-annotations: fls_fk2m2irwpeof
// Implementations
//
// ferrocene-annotations: fls_85vx1qfa061i
// Traits

//~ ERROR this file contains an unclosed delimiter
