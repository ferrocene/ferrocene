//@ revisions: thir post-mono post-mono-inlining
//@[thir] check-pass
//@[post-mono] build-fail
//@[post-mono] compile-flags: -Z inline-mir=no
//@[post-mono-inlining] build-fail
//@[post-mono-inlining] compile-flags: -Z inline-mir=yes

#![deny(ferrocene::unvalidated)]

#[inline(always)]
pub const fn trailing_ones(_: u32) -> u32 {
    0
}

#[ferrocene::prevalidated]
fn call(x: u32, f: impl FnOnce(u32) -> u32) {
    f(x);
    //[post-mono]~^ ERROR unvalidated
    //[post-mono-inlining]~^^ ERROR unvalidated
}

fn main() {
    call(123_u32, trailing_ones);
}
