fn take_str_maybe(x: Option<&str>) -> Option<&str> { None }

fn main() {
    let string = String::from("Hello, world");
    let option = Some(&string);
    take_str_maybe(option);
    //~^ ERROR: mismatched types [E0308]
}

// ferrocene-annotations: fls_lv7w7aalpwm5
// Type Inference
//
// ferrocene-annotations: fls_exe4zodlwfez
// Type Unification
