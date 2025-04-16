// Test for missing quotes around value, issue #66450.
//@ compile-flags: --cfg key=value

fn main() {}

<<<<<<< HEAD
// ferrocene-annotations: um_rustc_cfg
=======
//~? ERROR invalid `--cfg` argument: `key=value` (expected `key` or `key="value"`, ensure escaping is appropriate for your shell, try 'key="value"' or key=\"value\")
>>>>>>> pull-upstream-temp--do-not-use-for-real-code
