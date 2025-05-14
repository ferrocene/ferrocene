//@ compile-flags:-l raw-dylib=foo

fn main() {
}

//~? ERROR unknown library kind `raw-dylib`

// ferrocene-annotations: um_rustc_l
