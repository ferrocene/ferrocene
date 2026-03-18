static i: String = 10;
//~^ ERROR: mismatched types
//~| NOTE: expected `String`, found integer
//~| NOTE: expected because
fn main() { println!("{}", i); }

// ferrocene-annotations: fls_lv7w7aalpwm5
// Type Inference
//
// ferrocene-annotations: fls_exe4zodlwfez
// Type Unification
