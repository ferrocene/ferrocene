//@ build-fail
//@ dont-check-compiler-stderr
//@ compile-flags: -C linker=llllll

// Before, the error-pattern checked for "not found". On WSL with appendWindowsPath=true, running
// in invalid command returns a PermissionDenied instead.

fn main() {
}

<<<<<<< HEAD
// ferrocene-annotations: um_rustc_C_linker
=======
//~? ERROR `llllll`
>>>>>>> pull-upstream-temp--do-not-use-for-real-code
