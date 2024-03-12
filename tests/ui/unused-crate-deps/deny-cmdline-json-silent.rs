// Check for unused crate dep, json event, deny but we're not reporting that in exit status

//@ edition:2018
//@ check-pass
//@ compile-flags: -Dunused-crate-dependencies -Zunstable-options --json unused-externs-silent --error-format=json
//@ aux-crate:bar=bar.rs

fn main() {}

// ferrocene-annotations: um_rustc_json
// ferrocene-annotations: um_rustc_error_format
// ferrocene-annotations: um_rustc_D
