//@ build-pass
//@ ignore-pass (different metadata emitted in different modes)
//@ compile-flags: --json=diagnostic-short --json artifacts --error-format=json

#![crate_type = "lib"]

// ferrocene-annotations: um_rustc_json
// ferrocene-annotations: um_rustc_error_format
