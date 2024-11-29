//@ run-pass
#![allow(unused_parens)]

/* Make sure a loop{} can be the tailexpr in the body
of a diverging function */

fn forever() -> ! {
  loop{}
}

pub fn main() {
  if (1 == 2) { forever(); }
}

// ferrocene-annotations: fls_sf4qnd43z2wc
// Infinite Loops
//
// ferrocene-annotations: fls_98lnexk53ru4
// Never Type
//
// ferrocene-annotations: fls_exe4zodlwfez
// Type Unification
