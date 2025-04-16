//@ compile-flags: --cfg a(b=c)

fn main() {}

<<<<<<< HEAD
// ferrocene-annotations: um_rustc_cfg
=======
//~? ERROR invalid `--cfg` argument: `a(b=c)` (expected `key` or `key="value"`, ensure escaping is appropriate for your shell, try 'key="value"' or key=\"value\")
>>>>>>> pull-upstream-temp--do-not-use-for-real-code
