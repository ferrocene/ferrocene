// link-arg is not supposed to be usable in #[link] attributes

// compile-flags:
// error-pattern: error[E0458]: unknown link kind `link-arg`, expected one of: static, dylib, framework, raw-dylib

#[link(kind = "link-arg")]
extern "C" {}
pub fn main() {}

// ferrocene-annotations: fls_o0f9ae22ug1x
// Attribute link
