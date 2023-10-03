// Regression test to check that `'b: '_` gets an error, because it's
// basically useless.
//
// #54902

trait Foo<'a> {}
impl<'b: '_> Foo<'b> for i32 {} //~ ERROR `'_` cannot be used here
fn main() { }

// ferrocene-annotations: fls_85vx1qfa061i
// Traits
//
// ferrocene-annotations: fls_fk2m2irwpeof
// Implementations
