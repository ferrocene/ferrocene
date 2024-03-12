// Make sure that we don't parse `extern crate async`
// the front matter of a function leading us astray.

//@ check-pass

fn main() {}

#[cfg(FALSE)]
extern crate async;

#[cfg(FALSE)]
extern crate async as something_else;

// ferrocene-annotations: fls_gklst7joeo33
// External Crates
