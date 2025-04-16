//@ compile-flags:-D bogus
//@ check-pass
//@ dont-require-annotations: NOTE

fn main() {}

//~? WARN unknown lint: `bogus`
//~? WARN unknown lint: `bogus`
//~? WARN unknown lint: `bogus`
<<<<<<< HEAD

// ferrocene-annotations: um_rustc_D
=======
//~? NOTE requested on the command line with `-D bogus`
//~? NOTE `#[warn(unknown_lints)]` on by default
>>>>>>> pull-upstream-temp--do-not-use-for-real-code
