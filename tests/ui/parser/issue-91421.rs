// Regression test for issue #91421.

fn main() {
    let value = if true && {
    //~^ ERROR: this `if` expression is missing a block after the condition
    //~| HELP: this binary operation is possibly unfinished
        3
    } else { 4 };
}

// ferrocene-annotations: fls_mkut7gut49gi
// If Expressions
