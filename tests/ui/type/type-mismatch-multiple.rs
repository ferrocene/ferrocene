// Checking that the compiler reports multiple type errors at once

//@ dont-require-annotations: NOTE

fn main() { let a: bool = 1; let b: i32 = true; }
//~^ ERROR mismatched types
//~| NOTE expected `bool`, found integer
//~| ERROR mismatched types
<<<<<<< HEAD
//~| expected `i32`, found `bool`

// ferrocene-annotations: fls_exe4zodlwfez
// Type Unification
=======
//~| NOTE expected `i32`, found `bool`
>>>>>>> pull-upstream-temp--do-not-use-for-real-code
