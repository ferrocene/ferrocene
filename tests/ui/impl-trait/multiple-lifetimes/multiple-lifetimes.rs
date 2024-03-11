// Test that multiple lifetimes are allowed in impl trait types.
//@ build-pass (FIXME(62277): could be check-pass?)

trait X<'x>: Sized {}

impl<U> X<'_> for U {}

fn multiple_lifeteimes<'a, 'b, T: 'static>(x: &'a mut &'b T) -> impl X<'b> + 'a {
    x
}

fn main() {}

// ferrocene-annotations: fls_yqcygq3y6m5j
// Lifetime
//
// ferrocene-annotations: fls_142vncdktbin
// Reference Type
