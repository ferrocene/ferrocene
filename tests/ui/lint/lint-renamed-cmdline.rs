//@ compile-flags:-D bare_trait_object
//@ dont-require-annotations: NOTE

#[deny(unused)]
fn main() { let unused = (); } //~ ERROR unused variable: `unused`

//~? WARN lint `bare_trait_object` has been renamed to `bare_trait_objects`
//~? WARN lint `bare_trait_object` has been renamed to `bare_trait_objects`
//~? WARN lint `bare_trait_object` has been renamed to `bare_trait_objects`
<<<<<<< HEAD

// ferrocene-annotations: um_rustc_D
=======
//~? NOTE requested on the command line with `-D bare_trait_object`
//~? NOTE `#[warn(renamed_and_removed_lints)]` on by default
>>>>>>> pull-upstream-temp--do-not-use-for-real-code
