//@ compile-flags:-D bogus
//@ check-pass

//@ error-pattern:requested on the command line with `-D bogus`
//@ error-pattern:`#[warn(unknown_lints)]` on by default

fn main() {}

<<<<<<< HEAD
// ferrocene-annotations: um_rustc_D
=======
//~? WARN unknown lint: `bogus`
//~? WARN unknown lint: `bogus`
//~? WARN unknown lint: `bogus`
>>>>>>> pull-upstream-temp--do-not-use-for-real-code
