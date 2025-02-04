//@ build-pass
//@ ignore-pass (different metadata emitted in different modes)
//@ compile-flags: --json=diagnostic-short,artifacts --error-format=json
//@ normalize-stderr: "json-options\..+/libjson_options.rlib" -> "json-options/libjson_options.rlib"

#![crate_type = "lib"]

// ferrocene-annotations: um_rustc_json
// ferrocene-annotations: um_rustc_error_format
