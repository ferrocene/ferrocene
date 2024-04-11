#[allow(unused)]
fn foo() { //~ HELP try adding a return type
    vec!['a'].iter().map(|c| c)
    //~^ ERROR mismatched types [E0308]
    //~| NOTE expected `()`, found `Map<Iter<'_, char>, ...>`
    //~| NOTE expected unit type `()`
    //~| HELP consider using a semicolon here
}

fn main() {}

// ferrocene-annotations: fls_lv7w7aalpwm5
// Type Inference
//
// ferrocene-annotations: fls_exe4zodlwfez
// Type Unification
