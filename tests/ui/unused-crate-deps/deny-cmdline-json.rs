// Check for unused crate dep, json event, deny, expect compile failure

//@ edition:2018
//@ compile-flags: -Dunused-crate-dependencies  -Zunstable-options --json unused-externs --error-format=json
//@ aux-crate:bar=bar.rs

fn main() {}

// ferrocene-annotations: um_rustc_json
// ferrocene-annotations: um_rustc_error_format
// ferrocene-annotations: um_rustc_D
