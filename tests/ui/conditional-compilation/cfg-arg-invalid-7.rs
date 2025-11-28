// Regression test for issue #89358.

//@ compile-flags: --cfg a"

//~? RAW unterminated double quote string
<<<<<<< HEAD
//~? RAW this error occurred on the command line

// ferrocene-annotations: um_rustc_cfg
||||||| d2f887349fe
//~? RAW this error occurred on the command line
=======
//~? RAW this occurred on the command line
>>>>>>> pull-upstream-temp--do-not-use-for-real-code
