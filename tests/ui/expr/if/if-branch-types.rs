fn main() {
    let x = if true { 10i32 } else { 10u32 };
    //~^ ERROR `if` and `else` have incompatible types
    //~| expected `i32`, found `u32`
}

// ferrocene-annotations: fls_exe4zodlwfez
// Type Unification
// ferrocene-annotations: fls_mkut7gut49gi
// If Expressions
