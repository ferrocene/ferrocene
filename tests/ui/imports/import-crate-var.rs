//@ aux-build:import_crate_var.rs

#[macro_use] extern crate import_crate_var;

fn main() {
    m!();
    //~^ ERROR imports need to be explicitly named
}

// ferrocene-annotations: fls_gklst7joeo33
// External Crates
