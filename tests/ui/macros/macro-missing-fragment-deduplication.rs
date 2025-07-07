//@ compile-flags: -Zdeduplicate-diagnostics=yes

macro_rules! m {
    ($name) => {}; //~ ERROR missing fragment
}

fn main() {
    m!(); //~ ERROR unexpected end
    m!(); //~ ERROR unexpected end
    m!(); //~ ERROR unexpected end
    m!(); //~ ERROR unexpected end
}

// ferrocene-annotations: fls_xa7lp0zg1ol2
// Declarative Macros
//
// ferrocene-annotations: fls_vnvt40pa48n8
// Macro Invocation
//
// ferrocene-annotations: fls_8nzypdu9j3ge
// Metavariables
