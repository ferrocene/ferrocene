macro_rules! foo {
    { $+ } => { //~ ERROR expected identifier, found `+`
                //~^ ERROR missing fragment specifier
        $(x)(y) //~ ERROR expected one of: `*`, `+`, or `?`
    }
}

foo!(); //~ ERROR unexpected end

fn main() {}

// ferrocene-annotations: fls_k01lsksqtq1r
// Repetition
