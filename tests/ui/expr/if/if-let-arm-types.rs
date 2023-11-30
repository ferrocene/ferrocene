fn main() {
    if let Some(b) = None {
        //~^ NOTE `if` and `else` have incompatible types
        ()
        //~^ NOTE expected because of this
    } else {
        1
    };
    //~^^ ERROR: `if` and `else` have incompatible types
    //~| NOTE expected `()`, found integer
}

// ferrocene-annotations: fls_exe4zodlwfez
// Type Unification
// ferrocene-annotations: fls_p0t1ch115tra
// If Let Expressions
