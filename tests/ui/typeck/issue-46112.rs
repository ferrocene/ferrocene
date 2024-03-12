// Issue 46112: An extern crate pub re-exporting libcore was causing
// paths rooted from `std` to be misrendered in the diagnostic output.

//@ ignore-windows
//@ aux-build:xcrate-issue-46112-rexport-core.rs

extern crate xcrate_issue_46112_rexport_core;
fn test(r: Result<Option<()>, &'static str>) { }
fn main() { test(Ok(())); }
//~^ mismatched types

// ferrocene-annotations: fls_lv7w7aalpwm5
// Type Inference
//
// ferrocene-annotations: fls_exe4zodlwfez
// Type Unification
