fn strip_lf(s: &str) -> &str {
    s.strip_suffix(b'\n').unwrap_or(s)
    //~^ ERROR the trait bound `u8: Pattern` is not satisfied
    //~| NOTE required by a bound introduced by this call
    //~| NOTE the trait `FnMut(char)` is not implemented for `u8`
    //~| HELP the following other types implement trait `Pattern`:
    //~| NOTE required for `u8` to implement `Pattern`
    //~| NOTE required by a bound in `core::str::<impl str>::strip_suffix`
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
