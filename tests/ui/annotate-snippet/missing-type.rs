//@ compile-flags: --error-format human-annotate-rs -Z unstable-options

pub fn main() {
    let x: Iter;
}

<<<<<<< HEAD
// ferrocene-annotations: um_rustc_error_format
=======
//~? RAW cannot find type `Iter` in this scope
>>>>>>> pull-upstream-temp--do-not-use-for-real-code
