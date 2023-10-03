fn foo() -> i32 {
    4
}
fn main() {
    let x: u16 = foo();
    //~^ ERROR mismatched types
    let y: i64 = x + x;
    //~^ ERROR mismatched types
    let z: i32 = x + x;
    //~^ ERROR mismatched types
}

// ferrocene-annotations: fls_lv7w7aalpwm5
// Type Inference
//
// ferrocene-annotations: fls_exe4zodlwfez
// Type Unification
