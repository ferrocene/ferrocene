fn forever2() -> i32 {
  let x: i32 = loop { break }; //~ ERROR mismatched types
  x
}

fn main() {}

// ferrocene-annotations: fls_jr4tpuyksr75
// Break Expressions
//
// ferrocene-annotations: fls_sf4qnd43z2wc
// Infinite Loops
//
// ferrocene-annotations: fls_lv7w7aalpwm5
// Type Inference
//
// ferrocene-annotations: fls_exe4zodlwfez
// Type Unification
//
// ferrocene-annotations: fls_zjoamsr3dbqk
// Diverging Expressions
