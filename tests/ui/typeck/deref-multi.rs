fn a(x: &&i32) -> i32 {
    x
    //~^ ERROR mismatched types
}

fn a2(x: &&&&&i32) -> i32 {
    x
    //~^ ERROR mismatched types
}

fn b(x: &i32) -> i32 {
    &x
    //~^ ERROR mismatched types
}

fn c(x: Box<i32>) -> i32 {
    &x
    //~^ ERROR mismatched types
}

fn d(x: std::sync::Mutex<&i32>) -> i32 {
    x.lock().unwrap()
    //~^ ERROR mismatched types
}

fn main() {}

// ferrocene-annotations: fls_lv7w7aalpwm5
// Type Inference
//
// ferrocene-annotations: fls_exe4zodlwfez
// Type Unification
//
// ferrocene-annotations: fls_142vncdktbin
// Reference Types
