//~ ERROR unknown lint: `test_unstable_lint`
//@ check-fail
//@ compile-flags: -Dunknown_lints -Atest_unstable_lint
//@ error-pattern: the `test_unstable_lint` lint is unstable

fn main() {}

// ferrocene-annotations: um_rustc_A
// ferrocene-annotations: um_rustc_D
