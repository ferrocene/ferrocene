macro_rules! test {
    ($a, $b) => {
        //~^ ERROR missing fragment
        //~| ERROR missing fragment
        ()
    };
}

fn main() {
    test!() //~ ERROR unexpected end of macro invocation
}

// ferrocene-annotations: fls_xa7lp0zg1ol2
// Declarative Macros
//
// ferrocene-annotations: fls_4apk1exafxii
// Macro Matching
//
// ferrocene-annotations: fls_8nzypdu9j3ge
// Metavariables
//
// ferrocene-annotations: fls_n3ktmjqf87qb
// Rule Matching
//
// ferrocene-annotations: fls_qpx6lgapce57
// Token Matching
