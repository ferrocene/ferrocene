//@ build-pass
//@ ignore-pass (different metadata emitted in different modes)
//@ compile-flags: --json=diagnostic-short --json artifacts --error-format=json
//@ normalize-stderr: "json-multiple\..+/libjson_multiple.rlib" -> "json-multiple/libjson_multiple.rlib"

#![crate_type = "lib"]

// ferrocene-annotations: um_rustc_json
// ferrocene-annotations: um_rustc_error_format
