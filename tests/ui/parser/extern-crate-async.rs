//@ edition: 2015
// Make sure that we don't parse `extern crate async`
// the front matter of a function leading us astray.

//@ check-pass

fn main() {}

#[cfg(false)]
extern crate async;

#[cfg(false)]
extern crate async as something_else;

// ferrocene-annotations: fls_gklst7joeo33
// External Crates
