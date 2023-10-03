// Verifies that the expected token errors for `extern crate` are
// raised

extern crate foo {} //~ERROR expected one of `;` or `as`, found `{`

// ferrocene-annotations: fls_gklst7joeo33
// External Crates
