fn f() -> isize {
    (return 1, return 2)
//~^ ERROR mismatched types
//~| expected type `isize`
//~| found tuple `(!, !)`
//~| expected `isize`, found `(!, !)`
}

fn main() {}

// ferrocene-annotations: fls_lv7w7aalpwm5
// Type Inference
// ferrocene-annotations: fls_exe4zodlwfez
// Type Unification
