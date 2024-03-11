//@ compile-flags: --cfg foo

#[cfg(all(foo, bar))] // foo AND bar
fn foo() {}

fn main() {
    foo(); //~ ERROR cannot find function `foo` in this scope
}

// ferrocene-annotations: fls_fymvsy6ig99a
// Attribute cfg
//
// ferrocene-annotations: um_rustc_cfg
