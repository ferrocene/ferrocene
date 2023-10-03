#![allow(unused_macros)]

macro_rules! test { ($wrong:t_ty ..) => () }
                  //~^ ERROR: invalid fragment specifier `t_ty`

fn main() {}

// ferrocene-annotations: fls_xa7lp0zg1ol2
// Declarative Macros
//
// ferrocene-annotations: fls_8nzypdu9j3ge
// Metavariables
