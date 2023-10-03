macro_rules! mac {
    ( $($v:tt)* ) => {
        $v
        //~^ ERROR still repeating at this depth
        //~| ERROR still repeating at this depth
    };
}

fn main() {
    mac!(0);
    mac!(1);
}

// ferrocene-annotations: fls_8nzypdu9j3ge
// Metavariables
//
// ferrocene-annotations: fls_k01lsksqtq1r
// Repetition
