// (This line has BOM so it's ignored by compiletest for directives)
//
//@ compile-flags: --json=diagnostic-short --error-format=json
//@ reference: input.byte-order-mark
//@ reference: input.crlf
// ignore-tidy-cr

#[path = "json-bom-plus-crlf-multifile-aux.rs"]
mod json_bom_plus_crlf_multifile_aux;

fn main() {
    json_bom_plus_crlf_multifile_aux::test();
}

// ferrocene-annotations: um_rustc_json
// ferrocene-annotations: um_rustc_error_format
