//@ check-pass
//@ compile-flags:-D bogus -D dead_cod
//@ dont-require-annotations: HELP
//@ dont-require-annotations: NOTE

fn main() { }

//~? WARN unknown lint: `bogus`
//~? WARN unknown lint: `dead_cod`
//~? WARN unknown lint: `bogus`
//~? WARN unknown lint: `dead_cod`
//~? WARN unknown lint: `bogus`
//~? WARN unknown lint: `dead_cod`
<<<<<<< HEAD

// ferrocene-annotations: um_rustc_D
=======
//~? NOTE requested on the command line with `-D bogus`
//~? NOTE `#[warn(unknown_lints)]` on by default
//~? NOTE requested on the command line with `-D dead_cod`
//~? HELP did you mean: `dead_code`
>>>>>>> pull-upstream-temp--do-not-use-for-real-code
