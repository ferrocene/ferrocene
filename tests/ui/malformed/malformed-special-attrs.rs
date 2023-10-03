#[cfg_attr] //~ ERROR malformed `cfg_attr` attribute
struct S1;

#[cfg_attr = ""] //~ ERROR malformed `cfg_attr` attribute
struct S2;

#[derive] //~ ERROR malformed `derive` attribute
struct S3;

#[derive = ""] //~ ERROR malformed `derive` attribute
struct S4;

fn main() {}

// ferrocene-annotations: fls_dd9xh3wdjudo
// Attribute cfg_attr
//
// ferrocene-annotations: fls_r6gj1p4gajnq
// Attribute derive
