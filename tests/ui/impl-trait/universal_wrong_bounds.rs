use std::fmt::Display;

fn foo(f: impl Display + Clone) -> String {
    wants_debug(f);
    wants_display(f);
    wants_clone(f);
}

fn wants_debug(g: impl Debug) { } //~ ERROR expected trait, found derive macro `Debug`
fn wants_display(g: impl Debug) { } //~ ERROR expected trait, found derive macro `Debug`
fn wants_clone(g: impl Clone) { }

fn main() {}

// ferrocene-annotations: fls_jeoas4n6su4
// Trait and Lifetime Bounds
//
// ferrocene-annotations: fls_exe4zodlwfez
// Type Unification
