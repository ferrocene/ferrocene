// Regression test for issue #89358.

<<<<<<< HEAD
// compile-flags: --cfg a"
// error-pattern: unterminated double quote string
// error-pattern: this error occurred on the command line

// ferrocene-annotations: um_rustc_cfg
=======
//@ compile-flags: --cfg a"
//@ error-pattern: unterminated double quote string
//@ error-pattern: this error occurred on the command line
>>>>>>> pull-upstream-temp--do-not-use-for-real-code
