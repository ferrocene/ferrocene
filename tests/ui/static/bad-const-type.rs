static i: String = 10;
//~^ ERROR mismatched types
//~| expected `String`, found integer
fn main() { println!("{}", i); }

// ferrocene-annotations: fls_lv7w7aalpwm5
// Type Inference
//
// ferrocene-annotations: fls_exe4zodlwfez
// Type Unification
