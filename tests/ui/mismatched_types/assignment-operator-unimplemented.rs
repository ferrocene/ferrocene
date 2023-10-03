struct Foo;

fn main() {
  let mut a = Foo;
  let ref b = Foo;
  a += *b; //~ Error: binary assignment operation `+=` cannot be applied to type `Foo`
}

// ferrocene-annotations: fls_290jmzfh7x4e
// Compound Assignment Expressions
//
// ferrocene-annotations: fls_izdv9i4spokw
// Operator Expressions
