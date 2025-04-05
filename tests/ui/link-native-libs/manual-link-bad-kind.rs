//@ compile-flags:-l bar=foo

fn main() {
}

<<<<<<< HEAD
// ferrocene-annotations: um_rustc_l
=======
//~? ERROR unknown library kind `bar`
>>>>>>> pull-upstream-temp--do-not-use-for-real-code
