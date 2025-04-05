//@ compile-flags: --cfg a=10

fn main() {}

<<<<<<< HEAD
// ferrocene-annotations: um_rustc_cfg
=======
//~? ERROR invalid `--cfg` argument: `a=10` (argument value must be a string)
>>>>>>> pull-upstream-temp--do-not-use-for-real-code
