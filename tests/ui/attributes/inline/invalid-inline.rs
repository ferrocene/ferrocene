#![allow(dead_code)]

#[inline(please,no)] //~ ERROR malformed `inline` attribute
fn a() {
}

#[inline()] //~ ERROR malformed `inline` attribute
fn b() {
}

fn main() {
    a();
    b();
}

// ferrocene-annotations: fls_ypio6boj3pwf
// Attribute inline
