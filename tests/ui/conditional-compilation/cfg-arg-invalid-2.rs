//@ compile-flags: --cfg a{b}

fn main() {}

<<<<<<< HEAD
// ferrocene-annotations: um_rustc_cfg
=======
//~? ERROR invalid `--cfg` argument: `a{b}` (expected `key` or `key="value"`)
>>>>>>> pull-upstream-temp--do-not-use-for-real-code
