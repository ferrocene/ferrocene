//@ compile-flags: -F deprecated

#[allow(deprecated)] //~ ERROR allow(deprecated) incompatible with previous forbid [E0453]
fn main() {
}

// ferrocene-annotations: um_rustc_F
