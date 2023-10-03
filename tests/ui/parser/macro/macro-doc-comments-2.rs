macro_rules! inner {
    (#![$inner:meta]) => ()
}

inner! {
    /// Outer
} //~^ ERROR no rules expected the token `[`

fn main() { }

// ferrocene-annotations: fls_8nzypdu9j3ge
// Metavariables
