macro_rules! outer {
    (#[$outer:meta]) => ()
}

outer! {
    //! Inner
} //~^ ERROR no rules expected `!`

fn main() { }

// ferrocene-annotations: fls_8nzypdu9j3ge
// Metavariables
