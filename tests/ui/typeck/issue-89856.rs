//@ run-rustfix

fn take_str_maybe(_: Option<&str>) { }
fn main() {
    let string = String::from("Hello, world");

    let option: Option<String> = Some(string.clone());
    take_str_maybe(option);
    //~^ ERROR: mismatched types [E0308]

    let option_ref = Some(&string);
    take_str_maybe(option_ref);
    //~^ ERROR: mismatched types [E0308]

    let option_ref_ref = option_ref.as_ref();
    take_str_maybe(option_ref_ref);
    //~^ ERROR: mismatched types [E0308]
}

// ferrocene-annotations: fls_lv7w7aalpwm5
// Type Inference
//
// ferrocene-annotations: fls_exe4zodlwfez
// Type Unification
