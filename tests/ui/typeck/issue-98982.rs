fn foo() -> i32 { //~ HELP otherwise consider changing the return type to account for that possibility
    for i in 0..0 { //~ ERROR mismatched types [E0308]
        return i;
    } //~ HELP return a value for the case when the loop has zero elements to iterate on
}

fn main() {}

// ferrocene-annotations: fls_lv7w7aalpwm5
// Type Inference
//
// ferrocene-annotations: fls_exe4zodlwfez
// Type Unification
