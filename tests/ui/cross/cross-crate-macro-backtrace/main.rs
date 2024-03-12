//@ aux-build:extern_macro_crate.rs
#[macro_use(myprintln, myprint)]
extern crate extern_macro_crate;

fn main() {
    myprintln!("{}");
    //~^ ERROR in format string
}

// ferrocene-annotations: fls_qxjy0f758x5s
// Attribute macro_use
