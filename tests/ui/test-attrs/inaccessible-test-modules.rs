//@ compile-flags:--test

// the `--test` harness creates modules with these textual names, but
// they should be inaccessible from normal code.
use main as x; //~ ERROR unresolved import `main`
use test as y; //~ ERROR unresolved import `test`

#[test]
fn baz() {}

// ferrocene-annotations: fls_9gprp17h6t1q
// Use Imports
//
// ferrocene-annotations: um_rustc_test
