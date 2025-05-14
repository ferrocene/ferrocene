//@ compile-flags:-l bar=foo

fn main() {
}

//~? ERROR unknown library kind `bar`

// ferrocene-annotations: um_rustc_l
