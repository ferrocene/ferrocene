//@ run-pass

pub fn main() {
  let _i = 0_usize;
  loop {
    break;
  }
  assert!(true);
}

// ferrocene-annotations: fls_jr4tpuyksr75
// Break Expressions
//
// ferrocene-annotations: fls_sf4qnd43z2wc
// Infinite Loops
//
// ferrocene-annotations: fls_zjoamsr3dbqk
// Diverging Expressions
