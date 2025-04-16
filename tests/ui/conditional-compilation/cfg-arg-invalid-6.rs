//@ compile-flags: --cfg a{

fn main() {}

<<<<<<< HEAD
// ferrocene-annotations: um_rustc_cfg
=======
//~? ERROR invalid `--cfg` argument: `a{` (expected `key` or `key="value"`)
>>>>>>> pull-upstream-temp--do-not-use-for-real-code
