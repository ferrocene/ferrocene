//@ error-pattern: this file contains an unclosed delimiter
//@ error-pattern: mismatched closing delimiter: `]`

#![crate_name="0"]



fn main() {}

fn r()->i{0|{#[cfg(r(0{]0

// ferrocene-annotations: fls_qcb1n9c0e5hz
// Functions
