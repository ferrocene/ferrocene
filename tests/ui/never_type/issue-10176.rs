fn f() -> isize { //~ NOTE expected `isize` because of return type
    (return 1, return 2)
//~^ ERROR mismatched types
//~| NOTE expected type `isize`
//~| NOTE found tuple `(!, !)`
//~| NOTE expected `isize`, found `(!, !)`
}

fn main() {}

// ferrocene-annotations: fls_lv7w7aalpwm5
// Type Inference
// ferrocene-annotations: fls_exe4zodlwfez
// Type Unification
