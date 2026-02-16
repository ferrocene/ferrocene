<<<PULL-UPSTREAM>>> file deleted upstream; move the Ferrocene annotations if any, and delete this file
//@ compile-flags: -F deprecated

#[allow(deprecated)] //~ ERROR allow(deprecated) incompatible
fn main() {
}

// ferrocene-annotations: um_rustc_F
