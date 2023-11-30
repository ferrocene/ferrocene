#![feature(lang_items)]

fn main() {}

#![lang = "foo"] //~ ERROR an inner attribute is not permitted in this context
fn foo() {}

// ferrocene-annotations: fls_gvwd0kf72jt
// Attributes
