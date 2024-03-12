//@ compile-flags:-l bar=foo
//@ error-pattern: unknown library kind `bar`, expected one of: static, dylib, framework, link-arg

fn main() {
}

// ferrocene-annotations: um_rustc_l
