//@ check-pass
//@ compile-flags:-D bogus -D dead_cod

//@ error-pattern:requested on the command line with `-D bogus`
//@ error-pattern:`#[warn(unknown_lints)]` on by default
//@ error-pattern:requested on the command line with `-D dead_cod`
//@ error-pattern:did you mean: `dead_code`

fn main() { }

<<<<<<< HEAD
// ferrocene-annotations: um_rustc_D
=======
//~? WARN unknown lint: `bogus`
//~? WARN unknown lint: `dead_cod`
//~? WARN unknown lint: `bogus`
//~? WARN unknown lint: `dead_cod`
//~? WARN unknown lint: `bogus`
//~? WARN unknown lint: `dead_cod`
>>>>>>> pull-upstream-temp--do-not-use-for-real-code
