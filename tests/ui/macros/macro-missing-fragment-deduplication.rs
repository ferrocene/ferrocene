//@ compile-flags: -Zdeduplicate-diagnostics=yes

macro_rules! m {
    ($name) => {}
    //~^ ERROR missing fragment
    //~| ERROR missing fragment
    //~| WARN this was previously accepted
}

fn main() {
    m!();
    m!();
    m!();
    m!();
}

// ferrocene-annotations: fls_xa7lp0zg1ol2
// Declarative Macros
//
// ferrocene-annotations: fls_vnvt40pa48n8
// Macro Invocation
//
// ferrocene-annotations: fls_8nzypdu9j3ge
// Metavariables
