//@ edition:2018

// Built-in attribute
use inline as imported_inline;
mod builtin {
    pub use inline as imported_inline;
}

// Tool module
use rustfmt as imported_rustfmt;
mod tool_mod {
    pub use rustfmt as imported_rustfmt;
}

#[imported_inline] //~ ERROR cannot use a built-in attribute through an import
#[builtin::imported_inline] //~ ERROR cannot use a built-in attribute through an import
#[imported_rustfmt::skip] //~ ERROR cannot use a tool module through an import
                          //~| ERROR cannot use a tool module through an import
#[tool_mod::imported_rustfmt::skip] //~ ERROR cannot use a tool module through an import
                                    //~| ERROR cannot use a tool module through an import
fn main() {}

// ferrocene-annotations: fls_ahmnqhm8anlb
// Built-in Attributes
//
// ferrocene-annotations: fls_9gprp17h6t1q
// Use Imports
//
// ferrocene-annotations: fls_e9hwvqsib5d5
// Modules
//
// ferrocene-annotations: fls_gvwd0kf72jt
// Attributes
//
// ferrocene-annotations: fls_ld0ize96cm6m
// Preludes
