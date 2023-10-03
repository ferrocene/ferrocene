// gate-test-packed_bundled_libs

// ignore-wasm32-bare
// compile-flags: --crate-type rlib
// error-pattern: link modifiers combination `+bundle,+whole-archive` is unstable when generating rlibs
// build-fail

#[link(name = "rust_test_helpers", kind = "static", modifiers = "+bundle,+whole-archive")]
extern "C" {}

fn main() {}

// ferrocene-annotations: fls_o0f9ae22ug1x
// Attribute link
