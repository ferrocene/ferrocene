// Unspecified kind should fail with an error

//@ compile-flags: -l :+bundle=mylib
//@ error-pattern: unknown library kind ``, expected one of: static, dylib, framework, link-arg

fn main() {}

// ferrocene-annotations: um_rustc_l
