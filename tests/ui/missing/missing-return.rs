fn f() -> isize { } //~ ERROR mismatched types
                    //~| NOTE implicitly returns `()` as its body has no tail or `return` expression
                    //~| NOTE expected `isize`, found `()`
fn main() { f(); }

// ferrocene-annotations: fls_exe4zodlwfez
// Type Unification
