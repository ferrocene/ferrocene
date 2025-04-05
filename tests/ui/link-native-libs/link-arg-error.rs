//@ compile-flags: -l link-arg:+bundle=arg -Z unstable-options

fn main() {}

<<<<<<< HEAD
// ferrocene-annotations: um_rustc_l
=======
//~? ERROR linking modifier `bundle` is only compatible with `static` linking kind
>>>>>>> pull-upstream-temp--do-not-use-for-real-code
