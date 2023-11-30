// Regression test for the issue #44692

macro_rules! hang { () => {
    { //~ ERROR format argument must be a string literal
        #[derive(Clone)]
        struct S;

        ""
    }
}}

fn main() {
    format_args!(hang!());
}

// ferrocene-annotations: fls_r6gj1p4gajnq
// Attribute derive
//
// ferrocene-annotations: fls_xa7lp0zg1ol2
// Declarative Macros
