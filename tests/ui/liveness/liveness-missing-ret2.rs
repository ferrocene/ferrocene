fn f() -> isize { //~ ERROR mismatched types
    // Make sure typestate doesn't interpret this match expression as
    // the function result
   match true { true => { } _ => {} };
}

fn main() { }

// ferrocene-annotations: fls_lv7w7aalpwm5
// Type Inference
//
// ferrocene-annotations: fls_exe4zodlwfez
// Type Unification
