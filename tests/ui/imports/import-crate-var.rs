//@ aux-build:import_crate_var.rs

#[macro_use] extern crate import_crate_var;

fn main() {
    m!();
    //~^ ERROR `$crate` may not be imported
}

// ferrocene-annotations: fls_gklst7joeo33
// External Crates
