trait Foo<'a> {}

impl<'b: '_> Foo<'b> for i32 {} //~ ERROR `'_` cannot be used here

impl<T: '_> Foo<'static> for Vec<T> {} //~ ERROR `'_` cannot be used here

fn main() { }

// ferrocene-annotations: fls_85vx1qfa061i
// Traits
//
// ferrocene-annotations: fls_fk2m2irwpeof
// Implementations
//
// ferrocene-annotations: fls_e1pgdlv81vul
// Implementation Conformance
//
// ferrocene-annotations: fls_7nv8ualeaqe3
// Where Clauses
