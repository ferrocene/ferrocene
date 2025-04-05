//@ compile-flags:-D bare_trait_object

//@ error-pattern:requested on the command line with `-D bare_trait_object`
//@ error-pattern:`#[warn(renamed_and_removed_lints)]` on by default

#[deny(unused)]
<<<<<<< HEAD
fn main() { let unused = (); }

// ferrocene-annotations: um_rustc_D
=======
fn main() { let unused = (); } //~ ERROR unused variable: `unused`

//~? WARN lint `bare_trait_object` has been renamed to `bare_trait_objects`
//~? WARN lint `bare_trait_object` has been renamed to `bare_trait_objects`
//~? WARN lint `bare_trait_object` has been renamed to `bare_trait_objects`
>>>>>>> pull-upstream-temp--do-not-use-for-real-code
