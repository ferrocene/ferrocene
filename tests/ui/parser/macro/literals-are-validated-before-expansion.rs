macro_rules! black_hole {
    ($($tt:tt)*) => {}
}

fn main() {
    black_hole! { '\u{FFFFFF}' }
    //~^ ERROR: invalid unicode character escape
    black_hole! { "this is surrogate: \u{DAAA}" }
    //~^ ERROR: invalid unicode character escape
}

// ferrocene-annotations: fls_8nzypdu9j3ge
// Metavariables
//
// ferrocene-annotations: fls_k01lsksqtq1r
// Repetition
