// Regression test for issue #89358.

//@ compile-flags: --cfg a"
<<<<<<< HEAD
//@ error-pattern: unterminated double quote string
//@ error-pattern: this error occurred on the command line

// ferrocene-annotations: um_rustc_cfg
=======

//~? RAW unterminated double quote string
//~? RAW this error occurred on the command line
>>>>>>> pull-upstream-temp--do-not-use-for-real-code
