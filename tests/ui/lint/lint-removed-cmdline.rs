// The raw_pointer_derived lint warns about its removal
// cc #30346

//@ compile-flags:-D raw_pointer_derive
//@ dont-require-annotations: NOTE

#![warn(unused)]

#[deny(warnings)]
fn main() { let unused = (); } //~ ERROR unused variable: `unused`

//~? WARN lint `raw_pointer_derive` has been removed: using derive with raw pointers is ok
//~? WARN lint `raw_pointer_derive` has been removed: using derive with raw pointers is ok
//~? WARN lint `raw_pointer_derive` has been removed: using derive with raw pointers is ok
<<<<<<< HEAD

// ferrocene-annotations: um_rustc_D
=======
//~? NOTE `#[warn(renamed_and_removed_lints)]` on by default
//~? NOTE requested on the command line with `-D raw_pointer_derive`
>>>>>>> pull-upstream-temp--do-not-use-for-real-code
