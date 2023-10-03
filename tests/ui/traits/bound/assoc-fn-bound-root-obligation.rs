fn strip_lf(s: &str) -> &str {
    s.strip_suffix(b'\n').unwrap_or(s)
    //~^ ERROR expected a `FnMut<(char,)>` closure, found `u8`
    //~| NOTE expected an `FnMut<(char,)>` closure, found `u8`
    //~| HELP the trait `FnMut<(char,)>` is not implemented for `u8`
    //~| HELP the following other types implement trait `Pattern<'a>`:
    //~| NOTE required for `u8` to implement `Pattern<'_>`

}

fn main() {}

// ferrocene-annotations: fls_exe4zodlwfez
// Type Unification
//
// ferrocene-annotations: fls_jeoas4n6su4
// Trait and Lifetime Bounds
//
// ferrocene-annotations: fls_ikfvbeewame7
// Subtyping and Variance
