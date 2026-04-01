// This previously triggered an ICE.
//@ edition: 2015

pub(in crate::r#mod) fn main() {}
//~^ ERROR cannot find module or crate `r#mod` in `crate`
