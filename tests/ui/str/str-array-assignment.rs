fn main() {
  let s = "abc";
  let t = if true { s[..2] } else { s };
  //~^ ERROR `if` and `else` have incompatible types
  let u: &str = if true { s[..2] } else { s };
  //~^ ERROR mismatched types
  let v = s[..2];
  //~^ ERROR the size for values of type
  let w: &str = s[..2];
  //~^ ERROR mismatched types
}

// ferrocene-annotations: fls_exe4zodlwfez
// Type Unification
//
// ferrocene-annotations: fls_lv7w7aalpwm5
// Type Inference
