// This previously triggered an ICE.
//@ edition: 2015

pub(in crate::r#mod) fn main() {}
//~^ ERROR failed to resolve: use of unresolved module or unlinked crate `r#mod`
