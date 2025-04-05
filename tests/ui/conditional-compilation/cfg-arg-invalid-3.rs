//@ compile-flags: --cfg a::b

fn main() {}

<<<<<<< HEAD
// ferrocene-annotations: um_rustc_cfg
=======
//~? ERROR invalid `--cfg` argument: `a::b` (argument key must be an identifier)
>>>>>>> pull-upstream-temp--do-not-use-for-real-code
